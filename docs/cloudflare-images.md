# Media uploads on Cloudflare Images

This is separate from the [jobs migration](cloudflare-jobs.md). Images hosts and
transforms **new** uploads; it does not replace the queue consumer or change the
restored RWGPS OAuth, webhooks and history-sync mutation. No infrastructure or
production data is changed by this implementation. A missing enable flag disables
uploads. The checked-in configuration is now enabled for the next deployment,
following confirmation that the Images token and flexible variants are configured.

## Contracts and choices

- `POST /upload/media` keeps the existing frontend's authenticated multipart
  contract: one `file`, `name`, and JSON `relation_ids`. A valid Bearer login and
  ownership of every ride/route/trip/POI relation are required before contacting
  Images. Up to 20 relations and a 10,000,000-byte file are accepted. The whole
  multipart body has an additional 16 KiB allowance. Duplicate/unknown fields,
  invalid IDs, empty files, and unsupported MIME types fail closed.
- JPEG, PNG, GIF, WebP and HEIC/HEIF MIME types are accepted; Cloudflare validates
  the actual image and enforces pixel/animation limits. SVG is intentionally not
  accepted. The existing frontend picker remains unchanged. Images documents a
  10 MB limit, 12,000-pixel maximum dimension, 100 MP maximum area and 50 MP total
  animation area. Oversized originals are **not** downsampled in Wasm.
- The Worker buffers the bounded original, runs the existing metadata-only EXIF
  parser and forwards bytes unchanged. It never decodes, rotates, resizes or
  encodes pixels. This preserves the current client API and extracts metadata
  from the original, unlike a two-phase direct upload that would need another
  metadata source and a durable pending/finalization protocol.
- PostgreSQL retains ownership, relations, captured time and GPS. Missing EXIF
  is acceptable. Existing `nom-exif` timestamp interpretation is retained;
  timezone-less camera timestamps may not represent UTC correctly. This is not
  a timezone correction project. When captured time exists but GPS does not,
  the existing `MediaGeoInferrer` runs **before** the provider upload/save. Real
  EXIF GPS is not overwritten. This synchronous metadata lookup avoids a new
  DB-to-queue dual write; the queue/CLI remains available for later inference.
- A new `media.path` is a provider-qualified locator:
  `cloudflare-images://<public-delivery-hash>/<media-uuid>`. This fits the existing
  column, so new uploads need no schema migration. It is not a browser URL or
  an S3 key. GraphQL `path` exposes the locator; frontend image rendering already
  uses `imageSizes`, not `path`. The account API ID and token are not stored here.
- All seven GraphQL size fields and both `jpegUrl`/`webpUrl` fields remain intact.
  Images URLs use flexible variants: `cover` for 300/600 square crops and
  `scale-down` for 800/1200/1600/2000/2400 fits, explicit JPEG/WebP, quality 85,
  and `metadata=none`. Cloudflare handles EXIF orientation. Legacy records still
  produce exactly the old CloudFront URLs and keys.
- New image access is public (`requireSignedURLs=false`), like the existing
  public `howitt-media` S3 bucket/CloudFront distribution defined in
  `cdk/lib/howitt-media-stack.ts`. Flexible variants do not support signed images
  and apply account-wide. **`metadata=none` on app-generated URLs is not an
  access-control boundary**: a caller can request another flexible variant.
  Do not rely on this design for private photos or confidential GPS. A private
  media design needs separate authorization and delivery work.

## Provisioning and rollout — explicit approval required

Do not run these steps as part of local verification:

1. Confirm the intended Cloudflare account, Images billing/quotas and public
   access policy. Review account-wide flexible variants against other users of
   the same Images account. Enable hosted Images and flexible variants only
   with approval. No named variants or Images transformation binding are needed.
2. Create an account-scoped API token with **Images Write** permission for that
   account; do not reuse a broad deployment token. Store it as the web Worker's
   `IMAGES_API_TOKEN` secret. Never put it in Wrangler vars, frontend config or logs.
3. Set `IMAGES_ACCOUNT_ID` to the 32-character API account ID and
   `IMAGES_DELIVERY_HASH` to the public hash from Images Developer Resources.
   They are different identifiers. Configure request rate limits and monitor
   storage/delivery spending before enabling this authenticated upload endpoint;
   the application currently allows public signup.
4. After approval, deploy the web Worker with `IMAGES_UPLOADS_ENABLED="false"`
   first. Check existing CloudFront media and GraphQL. No jobs Worker deployment
   or DB migration is required. Then enable uploads in a separately approved
   deployment. Missing Images config leaves uploads unavailable; invalid config
   with the flag enabled fails Worker initialization, so validate it beforehand.
5. Use an explicitly approved test photo to verify live uploads, both response
   MIME types, crops, small-image no-upscale, orientation, PNG transparency,
   HEIC, animations and EXIF/GPS behavior. Confirm browser upload/refetch in the
   trip editor. Local mocks verify app behavior, not Cloudflare's actual codecs,
   provisioning or billing. No live image has been uploaded by these tests.

To stop new uploads, deploy with the flag false. Keep this version's delivery
resolver: rolling back to a pre-Images binary would break newly stored locators.
Do not delete Images originals, S3 objects or CloudFront infrastructure as a
rollback mechanism.

## Failure and reconciliation

A 201 is sent only after Images confirms HTTP success, `success=true`, the exact
requested UUID, and PostgreSQL commits the media and relations transaction.
There is no distributed transaction between Images and PostgreSQL. Timeouts,
ambiguous provider responses or a DB failure can leave an unlinked original.
No automated cleanup deletes that original. Images stores `creator` and
`metadata.media_id`; logs record media ID and failure stage without credentials,
provider response bodies or image metadata. Reconcile by that ID before retrying
an uncertain request. A manual retry currently receives a new media UUID and
can create a duplicate. The frontend does not automatically retry uploads.

For reconciliation, compare Images metadata with PostgreSQL in a read-only
inventory first. If a confirmed Images object has no row, retain it until an
operator can recover ownership, relations and metadata from the original/client.
Any database repair or deletion needs a reviewed, explicitly approved operation.
An outbox/pending upload ledger would be appropriate if automatic recovery or
idempotent client retries become requirements; neither is claimed here.

## Existing-media migration plan — no backfill is performed

The source currently defines `howitt-media` S3 originals at
`originals/user/<user-uuid>/media/<media-uuid>/<sanitized-name>` and resized files
at `resizes/user/<user-uuid>/media/<uuid>_<spec>.<ext>`, delivered through
`d36p712mevhglz.cloudfront.net`. This is a source-code contract, not a completed
inventory of the live bucket. Existing paths and objects are left untouched.

1. Obtain approval for a read-only production DB/bucket inventory and its storage
   destination. Export a restricted manifest with media UUID, owner, original
   path, S3 version/checksum/size, captured time and location. Flag missing
   originals and files exceeding Images limits; leave these on legacy delivery.
2. Implement a resumable copy tool using the native CLI or Images URL import,
   not a Wasm image processor. Preserve media UUIDs, use private/presigned origin
   access when needed, and never log presigned URLs. Upload originals only after
   approval. Track each confirmed Images ID/hash in the manifest. Do not replace
   originals with legacy thumbnails. Verify representative variants and metadata.
3. **Before any delivery switch**, add a durable provider mapping that retains
   the legacy original path (for example an additive mapping table). The new
   upload locator convention alone is not a sufficient migration audit trail.
   Prepare its schema migration and rollback, test them on a disposable copy,
   then obtain approval for production migration/backfill. Do not simply bulk
   overwrite `media.path` and lose the original storage reference.
4. Switch validated records in small, reversible batches. Preserve existing
   app-owned timestamps, locations and relations rather than re-infer them.
   Check public pages, mixed-provider galleries and costs. Keep missing,
   unsupported or failed objects on CloudFront. Roll back through the mapping.
5. Keep all S3 originals and resized assets throughout migration and rollback
   retention. Deleting originals or retiring S3/CloudFront is a separate decision
   requiring explicit approval, never a side effect of this work.

## Local verification

With the repo Rust toolchain, Bun, worker-build and local PostgreSQL/PostGIS:

```sh
amp orb services ensure
PG_USER=howitt_orb bash scripts/test-images-local.sh
cargo test --locked -p howitt-web --lib
cargo test --locked -p howitt services::media
cargo test --locked -p exif
cargo check --locked -p howitt-web --target wasm32-unknown-unknown
bun run check:worker
```

The script owns and drops a uniquely named loopback database, never reads
`DATABASE_URL`, uses synthetic credentials and blocks all real upstream calls.
It executes the compiled Wasm upload and GraphQL handlers, checking rejection
before side effects, provider failures, original bytes, EXIF capture/GPS and
inference, DB transaction failure, disabled config and mixed-provider URLs.
No frontend query or component changes are required.

## Cloudflare documentation consulted

- [Upload API](https://developers.cloudflare.com/api/resources/images/subresources/v1/methods/create/)
- [Upload formats and limits](https://developers.cloudflare.com/images/upload-images/)
- [Hosted delivery URLs](https://developers.cloudflare.com/images/optimization/hosted-images/serve-uploaded-images/)
- [Flexible variants](https://developers.cloudflare.com/images/optimization/hosted-images/enable-flexible-variants/)
- [Optimization parameters and metadata](https://developers.cloudflare.com/images/optimization/features/)
- [Named variant fit semantics](https://developers.cloudflare.com/images/optimization/hosted-images/create-variants/)

Reviewed September 2026. Account configuration and live codec behavior still
require the approved rollout checks above.
