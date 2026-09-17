import assert from "node:assert/strict";
import { resolve } from "node:path";
import { SQL } from "bun";
import { Miniflare, type MiniflareOptions } from "miniflare";

const database = process.env.HOWITT_TEST_DATABASE_URL;
assert(database, "Run through scripts/test-images-local.sh");
const url = new URL(database);
assert.equal(url.hostname, "127.0.0.1");
assert(url.pathname.startsWith("/howitt_workers_test_"));
const sql = new SQL(database);
const userId = "00000000-0000-0000-0000-000000000091";
const otherUser = "00000000-0000-0000-0000-000000000092";
const tripId = "00000000-0000-0000-0000-000000000093";
const otherTrip = "00000000-0000-0000-0000-000000000094";
const legacyId = "00000000-0000-0000-0000-000000000095";
const image = new Uint8Array(
  await Bun.file("webui/public/logo.jpg").arrayBuffer(),
);
// Minimal TIFF EXIF segment: timestamp and optional southern/eastern GPS.
// Build bytes independently of the Rust parser; append the real JPEG unchanged.
function withExif(gps: boolean): Uint8Array {
  const tiff = Buffer.alloc(178);
  tiff.write("II");
  tiff.writeUInt16LE(42, 2);
  tiff.writeUInt32LE(8, 4);
  const entry = (
    offset: number,
    tag: number,
    type: number,
    count: number,
    value: number,
  ) => {
    tiff.writeUInt16LE(tag, offset);
    tiff.writeUInt16LE(type, offset + 2);
    tiff.writeUInt32LE(count, offset + 4);
    tiff.writeUInt32LE(value, offset + 8);
  };
  tiff.writeUInt16LE(gps ? 2 : 1, 8);
  entry(10, 0x8769, 4, 1, 38);
  if (gps) entry(22, 0x8825, 4, 1, 56);
  tiff.writeUInt16LE(1, 38);
  entry(40, 0x9003, 2, 20, 158);
  tiff.write("2026:09:01 00:07:00\0", 158);
  tiff.writeUInt16LE(4, 56);
  entry(58, 1, 2, 2, 83);
  entry(70, 2, 5, 3, 110); // S, latitude
  entry(82, 3, 2, 2, 69);
  entry(94, 4, 5, 3, 134); // E, longitude
  [31, 30, 0, 115, 45, 0].forEach((value, i) => {
    tiff.writeUInt32LE(value, 110 + i * 8);
    tiff.writeUInt32LE(1, 114 + i * 8);
  });
  const segment = Buffer.concat([Buffer.from("Exif\0\0"), tiff]);
  const header = Buffer.from([0xff, 0xe1, 0, 0]);
  header.writeUInt16BE(segment.length + 2, 2);
  return new Uint8Array(
    Buffer.concat([image.slice(0, 2), header, segment, image.slice(2)]),
  );
}
let expectedBytes = image;
const uploads: string[] = [];
let failure: "none" | "status" | "envelope" | "id" = "none";
const options = {
  workers: [
    {
      config: {
        type: "worker",
        name: "images-test",
        compatibilityDate: "2026-09-12",
        compatibilityFlags: ["nodejs_als"],
        manifest: {
          mainModule: "index.js",
          modulesRoot: resolve("src/bin/howitt-web/build"),
          modules: {
            "index.js": {
              type: "esm",
              contents: await Bun.file(
                "src/bin/howitt-web/build/index.js",
              ).text(),
            },
            "index_bg.wasm": {
              type: "wasm",
              contents: new Uint8Array(
                await Bun.file(
                  "src/bin/howitt-web/build/index_bg.wasm",
                ).arrayBuffer(),
              ),
            },
          },
        },
        assets: {
          directory: resolve("src/bin/howitt-web/build/assets"),
          hasUserWorker: true,
          runWorkerFirst: true,
        },
        env: {
          HYPERDRIVE: {
            type: "hyperdrive",
            id: "local",
            dev: { connectionString: database },
          },
          DERIVED_CACHE: { type: "kv", id: "local-images" },
          ASSETS: { type: "assets" },
          JWT_SECRET: { type: "text", value: "local-test-signing-key" },
          JOBS: { type: "queue", name: "images-test-jobs" },
          RWGPS_CLIENT_ID: { type: "text", value: "local-rwgps-client" },
          RWGPS_CLIENT_SECRET: { type: "text", value: "local-rwgps-secret" },
          RWGPS_REDIRECT_URI: {
            type: "text",
            value: "https://images.test/auth/rwgps/callback",
          },
          IMAGES_UPLOADS_ENABLED: { type: "text", value: "true" },
          IMAGES_ACCOUNT_ID: { type: "text", value: "a".repeat(32) },
          IMAGES_DELIVERY_HASH: { type: "text", value: "test-hash" },
          IMAGES_API_TOKEN: { type: "text", value: "synthetic-images-token" },
        },
      },
      dev: {
        outboundService: {
          type: "fetcher",
          handler: async (request: Request) => {
            assert.equal(
              request.url,
              `https://api.cloudflare.com/client/v4/accounts/${"a".repeat(32)}/images/v1`,
              "No live external calls",
            );
            assert.equal(request.method, "POST");
            assert.equal(
              request.headers.get("authorization"),
              "Bearer synthetic-images-token",
            );
            const form = await request.formData();
            assert.equal(form.get("creator"), `USER#${userId}`);
            assert.equal(form.get("requireSignedURLs"), "false");
            const id = String(form.get("id"));
            // Images reserves bare UUIDs; the original implementation failed with 5411.
            if (
              /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(
                id,
              )
            ) {
              return Response.json(
                {
                  success: false,
                  errors: [
                    {
                      code: 5411,
                      message: "Custom ID is not valid: Must not be UUID",
                    },
                  ],
                },
                { status: 400 },
              );
            }
            assert.match(id, /^howitt-[0-9a-f-]{36}$/);
            assert.deepEqual(JSON.parse(String(form.get("metadata"))), {
              media_id: `MEDIA#${id.slice("howitt-".length)}`,
            });
            const file = form.get("file") as File;
            assert.equal(file.name, "test.jpg");
            assert.equal(file.type, "image/jpeg");
            assert.deepEqual(
              new Uint8Array(await file.arrayBuffer()),
              expectedBytes,
              "Original bytes must not be transformed",
            );
            uploads.push(id);
            return Response.json(
              {
                success: failure !== "envelope",
                errors:
                  failure === "envelope"
                    ? [{ code: 5411, message: "upstream-sensitive-message" }]
                    : [],
                result: { id: failure === "id" ? "wrong-id" : id },
              },
              { status: failure === "status" ? 503 : 200 },
            );
          },
        },
      },
    },
  ],
} satisfies MiniflareOptions;
const mf = new Miniflare(options);

try {
  const password = await Bun.password.hash("synthetic-password", {
    algorithm: "argon2id",
  });
  await sql`insert into users (id, username, password, email) values (${userId}, 'images-test', ${password}, 'images@example.invalid'), (${otherUser}, 'other-images-test', ${password}, 'other@example.invalid')`;
  await sql`insert into trips (id, user_id, name, slug, year) values (${tripId}, ${userId}, 'Images trip', 'images-trip', 2026), (${otherTrip}, ${otherUser}, 'Other trip', 'other-trip', 2026)`;
  await sql`insert into media (id, user_id, path) values (${legacyId}, ${userId}, 'originals/legacy.jpg')`;
  await sql`insert into trip_media (trip_id, media_id) values (${tripId}, ${legacyId})`;
  const worker = await mf.getWorker("images-test");
  const request = (path: string, init: RequestInit) =>
    worker.fetch(`http://images.test${path}`, init);
  const login = await request("/auth/login", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      username: "images-test",
      password: "synthetic-password",
    }),
  });
  const { token } = (await login.json()) as { token: string };
  assert.equal(typeof token, "string");
  const upload = (
    relations = [`TRIP#${tripId}`],
    mime = "image/jpeg",
    bytes = image,
    auth = token,
    duplicate = false,
  ) => {
    const form = new FormData();
    form.append("file", new Blob([bytes], { type: mime }), "test.jpg");
    form.append("name", "test.jpg");
    form.append("relation_ids", JSON.stringify(relations));
    if (duplicate) form.append("name", "extra.jpg");
    return request("/upload/media", {
      method: "POST",
      headers: { authorization: `Bearer ${auth}` },
      body: form,
    });
  };
  assert.equal(
    (await upload(undefined, undefined, undefined, "invalid")).status,
    401,
  );
  assert.equal((await upload([`TRIP#${otherTrip}`])).status, 403);
  assert.equal(
    (await upload(["TRIP#00000000-0000-0000-0000-000000000099"])).status,
    403,
  );
  assert.equal((await upload(["nonsense"])).status, 400);
  assert.equal((await upload([], "image/svg+xml")).status, 415);
  assert.equal((await upload([], "image/jpeg", new Uint8Array())).status, 400);
  assert.equal(
    (await upload([], "image/jpeg", new Uint8Array(10_000_001))).status,
    413,
  );
  assert.equal(
    (await upload([], "image/jpeg", image, token, true)).status,
    400,
  );
  assert.equal(
    uploads.length,
    0,
    "Rejected uploads cannot create Images objects",
  );
  for (const mode of ["status", "envelope", "id"] as const) {
    failure = mode;
    assert.equal((await upload()).status, 502);
  }
  assert.equal(
    (await sql`select count(*)::int as n from media`)[0].n,
    1,
    "Unconfirmed uploads cannot create media rows",
  );
  failure = "none";
  const response = await upload();
  assert.equal(response.status, 201, await response.clone().text());
  const { id } = (await response.json()) as { id: string };
  const mediaId = id.replace("MEDIA#", "");
  const [row] =
    await sql`select path, user_id, captured_at, point from media where id=${mediaId}`;
  assert.equal(row.path, `cloudflare-images://test-hash/howitt-${mediaId}`);
  assert.equal(row.user_id, userId);
  assert.equal(row.captured_at, null);
  assert.equal(row.point, null);
  const [relation] =
    await sql`select trip_id from trip_media where media_id=${mediaId}`;
  assert.equal(relation.trip_id, tripId);
  const gql = await request("/", {
    method: "POST",
    headers: {
      "content-type": "application/json",
      authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({
      query:
        "{ trips { media { id path imageSizes { fill300 { width height mode jpegUrl webpUrl } fit1200 { jpegUrl webpUrl } } } } }",
    }),
  });
  const result = (await gql.json()) as any;
  assert.equal(result.errors, undefined, JSON.stringify(result));
  const media = result.data.trips.flatMap((trip: any) => trip.media);
  const old = media.find((item: any) => item.id === `MEDIA#${legacyId}`);
  const fresh = media.find((item: any) => item.id === id);
  assert.equal(old.path, "originals/legacy.jpg");
  assert.equal(
    old.imageSizes.fill300.jpegUrl,
    `https://d36p712mevhglz.cloudfront.net/resizes/user/${userId}/media/${legacyId}_fill_300x300.jpg`,
  );
  assert.equal(
    old.imageSizes.fit1200.webpUrl,
    `https://d36p712mevhglz.cloudfront.net/resizes/user/${userId}/media/${legacyId}_fit_1200x1200.webp`,
  );
  assert.deepEqual(fresh.imageSizes.fill300, {
    width: 300,
    height: 300,
    mode: "FILL",
    jpegUrl: `https://imagedelivery.net/test-hash/howitt-${mediaId}/width=300,height=300,fit=cover,format=jpeg,quality=85,metadata=none`,
    webpUrl: `https://imagedelivery.net/test-hash/howitt-${mediaId}/width=300,height=300,fit=cover,format=webp,quality=85,metadata=none`,
  });
  assert.equal(
    fresh.imageSizes.fit1200.jpegUrl,
    `https://imagedelivery.net/test-hash/howitt-${mediaId}/width=1200,height=1200,fit=scale-down,format=jpeg,quality=85,metadata=none`,
  );

  const rideId = "00000000-0000-0000-0000-000000000096";
  await sql`insert into rides (id, user_id, name, distance_m, started_at, finished_at) values (${rideId}, ${userId}, 'Inference ride', 1000, '2026-09-01T00:00:00Z', '2026-09-01T00:10:00Z')`;
  const points = [
    [Date.parse("2026-09-01T00:00:00Z") / 1000, 116, -32, 10],
    [Date.parse("2026-09-01T00:10:00Z") / 1000, 117, -33, 20],
  ];
  await sql`insert into ride_points (ride_id, points) values (${rideId}, ${JSON.stringify(points)}::text::jsonb)`;
  assert.deepEqual(
    (await sql`select points from ride_points where ride_id=${rideId}`)[0]
      .points,
    points,
  );
  for (const gps of [false, true]) {
    expectedBytes = withExif(gps);
    const res = await upload(undefined, undefined, expectedBytes);
    assert.equal(res.status, 201, await res.clone().text());
    const body = (await res.json()) as { id: string };
    const [metadata] =
      await sql`select captured_at, point from media where id=${body.id.replace("MEDIA#", "")}`;
    assert.equal(
      metadata.captured_at.toISOString(),
      "2026-09-01T00:07:00.000Z",
    );
    assert.deepEqual(
      metadata.point,
      gps ? { x: 115.75, y: -31.5 } : { x: 117, y: -33 },
    );
  }
  expectedBytes = new Uint8Array(10_000_000);
  expectedBytes.set(image);
  assert.equal(
    (await upload([], "image/jpeg", expectedBytes)).status,
    201,
    "Exactly 10 MB is accepted",
  );
  expectedBytes = image;

  // Fail the DB write after a confirmed provider upload. Never delete its original.
  await sql`alter table media add constraint reject_new_images check (path not like 'cloudflare-images://%') not valid`;
  assert.equal((await upload()).status, 500);
  assert.equal((await sql`select count(*)::int as n from media`)[0].n, 5);
  await sql`alter table media drop constraint reject_new_images`;
  options.workers[0].config.env.IMAGES_UPLOADS_ENABLED.value = "false";
  const disabledMf = new Miniflare(options);
  try {
    const disabledWorker = await disabledMf.getWorker("images-test");
    const form = new FormData();
    form.append("name", "test.jpg");
    const disabled = await disabledWorker.fetch(
      "http://images.test/upload/media",
      {
        method: "POST",
        headers: { authorization: `Bearer ${token}` },
        body: form,
      },
    );
    assert.equal(disabled.status, 503);
  } finally {
    await disabledMf.dispose();
  }
  console.log(
    "Images Wasm integration: auth, ownership, multipart limits, upstream failures, persistence, EXIF/GPS/inference, original bytes, legacy/new GraphQL delivery, DB failure and disabled configuration passed",
  );
} finally {
  await mf.dispose();
  await sql.close();
}
