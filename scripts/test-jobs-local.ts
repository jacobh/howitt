import assert from "node:assert/strict";
import { resolve } from "node:path";
import { SQL } from "bun";
import { Miniflare, type MiniflareOptions } from "miniflare";

const database = process.env.HOWITT_TEST_DATABASE_URL;
assert(database, "Run through scripts/test-jobs-local.sh");
const url = new URL(database);
assert.equal(url.hostname, "127.0.0.1");
assert(url.pathname.startsWith("/howitt_workers_test_"));
const sql = new SQL(database);
const userId = "00000000-0000-0000-0000-000000000081";
const now = "2026-09-01T00:00:00Z";
const upstream = {
  type: "route",
  route: {
    id: 314,
    highlighted_photo_id: 0,
    highlighted_photo_checksum: null,
    distance: 12345,
    track_id: "synthetic",
    user_id: 81,
    pavement_type: null,
    pavement_type_id: null,
    // Live RWGPS routes no longer include recreation_type_ids or last coordinates.
    activity_types: ["cycling"],
    visibility: 0,
    created_at: now,
    updated_at: now,
    name: "Synthetic queue route",
    first_lng: 115.8,
    first_lat: -31.9,
    bounding_box: [],
    country_code: "AU",
    privacy_code: null,
    user: {
      id: 81,
      created_at: now,
      description: null,
      interests: null,
      account_level: 0,
      total_trip_distance: 0,
      total_trip_duration: 0,
      name: "Synthetic",
      highlighted_photo_id: 0,
      highlighted_photo_checksum: null,
    },
    has_course_points: false,
    tag_names: [],
    track_type: "cycling",
    terrain: "",
    difficulty: "",
    unpaved_pct: 0,
    surface: "",
    nav_enabled: false,
    rememberable: false,
    metrics: {},
    photos: [
      {
        id: 1,
        created_at: now,
        updated_at: now,
        visibility: 0,
        lat: null,
        lng: null,
        user_id: 81,
        width: 800,
        height: 600,
        optional_uuid: null,
        checksum: "synthetic-photo",
        // group_membership_id is absent in live route photos.
      },
    ],
    course_points: [],
    points_of_interest: [],
    track_points: [
      { x: 115.8, y: -31.9, e: 23 },
      { x: 115.9, y: -32, e: 47 },
    ],
  },
};
let expectedToken = "synthetic-old-token";
let requests = 0;
let upstreamFails = false;
const children: unknown[] = [];
const options = {
  workers: [
    {
      config: {
        type: "worker",
        name: "jobs",
        compatibilityDate: "2026-09-12",
        compatibilityFlags: ["nodejs_als"],
        manifest: {
          mainModule: "index.js",
          modulesRoot: resolve("src/bin/howitt-worker/build"),
          modules: {
            "index.js": {
              type: "esm",
              contents: await Bun.file(
                "src/bin/howitt-worker/build/index.js",
              ).text(),
            },
            "index_bg.wasm": {
              type: "wasm",
              contents: new Uint8Array(
                await Bun.file(
                  "src/bin/howitt-worker/build/index_bg.wasm",
                ).arrayBuffer(),
              ),
            },
          },
        },
        env: {
          HYPERDRIVE: {
            type: "hyperdrive",
            id: "local",
            dev: { connectionString: database },
          },
          JOBS: { type: "queue", name: "test-children" },
        },
      },
      dev: {
        outboundService: {
          type: "fetcher",
          handler: async (request: Request) => {
            const target = new URL(request.url);
            assert.equal(
              target.hostname,
              "ridewithgps.com",
              "No real external API calls in tests",
            );
            assert(
              ["/routes/314.json", "/trips/315.json"].includes(target.pathname),
            );
            assert.equal(target.searchParams.get("apikey"), "howitt");
            assert.equal(target.searchParams.get("version"), "2");
            assert.equal(target.searchParams.has("auth_token"), false);
            assert.equal(
              request.headers.get("authorization"),
              `Bearer ${expectedToken}`,
            );
            requests++;
            if (upstreamFails)
              return Response.json(
                { error: "synthetic outage" },
                { status: 503 },
              );
            if (target.pathname === "/trips/315.json") {
              return Response.json({
                type: "trip",
                trip: {
                  ...upstream.route,
                  id: 315,
                  last_lng: 115.9,
                  last_lat: -32,
                  locality: null,
                  postal_code: null,
                  administrative_area: null,
                  departed_at: now,
                  is_stationary: false,
                  live_logging: false,
                  live_log: null,
                  metrics: { grade: {} },
                  track_points: [
                    { x: 115.8, y: -31.9, e: 23, t: Date.parse(now) / 1000 },
                    {
                      x: 115.9,
                      y: -32,
                      e: 47,
                      t: Date.parse(now) / 1000 + 600,
                    },
                  ],
                },
              });
            }
            return Response.json(upstream);
          },
        },
      },
    },
    {
      config: {
        type: "worker",
        name: "children",
        compatibilityDate: "2026-09-12",
        manifest: {
          mainModule: "consumer.js",
          modules: {
            "consumer.js": {
              type: "esm",
              contents: `export default { async queue(batch) {
          await fetch("https://record.invalid", {method: "POST", body: JSON.stringify(batch.messages.map(m => m.body))});
        }};`,
            },
          },
        },
        triggers: [
          {
            type: "queue",
            name: "test-children",
            maxBatchSize: 1,
            maxBatchTimeout: 0,
          },
        ],
      },
      dev: {
        outboundService: {
          type: "fetcher",
          handler: async (request: Request) => {
            children.push(...((await request.json()) as unknown[]));
            return new Response("ok");
          },
        },
      },
    },
  ],
} satisfies MiniflareOptions;
const mf = new Miniflare(options);
let missingProducer: Miniflare | undefined;

try {
  const password = await Bun.password.hash("synthetic-password", {
    algorithm: "argon2id",
  });
  await sql`insert into users (id, username, password, email) values (${userId}, 'queue-runtime', ${password}, 'queue-runtime@example.invalid')`;
  await sql`insert into user_rwgps_connections (id, user_id, rwgps_user_id, access_token) values (${userId}, ${userId}, 81, ${expectedToken})`;
  const worker = await mf.getWorker("jobs");
  const routeJob = {
    version: "1",
    job: {
      Rwgps: { SyncRoute: { user_id: `USER#${userId}`, rwgps_route_id: 314 } },
    },
  };
  const deliver = (id: string, body: unknown) =>
    withinTimeout(
      worker.queue("howitt-jobs", [
        { id, timestamp: new Date(), body, attempts: 1 },
      ]),
    );
  const first = await deliver("first", routeJob);
  assert.deepEqual(first.explicitAcks, ["first"]);
  assert.deepEqual(first.retryMessages, []);
  const [saved] =
    await sql`select r.id, distance_m, p.points from routes r join route_points p on p.route_id=r.id where external_ref->'id'->'Rwgps'->>'Route'='314'`;
  assert.equal(saved.distance_m, 12345);
  assert.equal(saved.points.length, 2);
  assert.deepEqual(saved.points, [
    [115.8, -31.9, 23],
    [115.9, -32, 47],
  ]);

  // A queued ID resolves rotated credentials at execution time, not enqueue time.
  expectedToken = "synthetic-rotated-token";
  await sql`update user_rwgps_connections set access_token=${expectedToken} where user_id=${userId}`;
  const second = await deliver("duplicate", routeJob);
  assert.deepEqual(second.explicitAcks, ["duplicate"]);
  const rows =
    await sql`select id from routes where external_ref->'id'->'Rwgps'->>'Route'='314'`;
  assert.deepEqual(
    rows.map((row: { id: string }) => row.id),
    [saved.id],
  );
  assert.equal(requests, 2);

  const tripJob = {
    version: "1",
    job: {
      Rwgps: { SyncTrip: { user_id: `USER#${userId}`, rwgps_trip_id: 315 } },
    },
  };
  assert.deepEqual((await deliver("trip", tripJob)).explicitAcks, ["trip"]);
  const [ride] =
    await sql`select r.id, p.points from rides r join ride_points p on p.ride_id=r.id where external_ref->'id'->'Rwgps'->>'Trip'='315'`;
  assert.deepEqual(ride.points, [
    [Date.parse(now) / 1000, 115.8, -31.9, 23],
    [Date.parse(now) / 1000 + 600, 115.9, -32, 47],
  ]);
  const mediaId = "00000000-0000-0000-0000-000000000082";
  const capturedAt = new Date(Date.parse(now) + 500_000);
  await sql`insert into media (id, user_id, path, captured_at) values (${mediaId}, ${userId}, 'synthetic.jpg', ${capturedAt})`;
  const inference = await deliver("infer", {
    version: "1",
    job: { Media: { InferLocation: `MEDIA#${mediaId}` } },
  });
  assert.deepEqual(inference.explicitAcks, ["infer"]);
  const [media] = await sql`select point from media where id=${mediaId}`;
  assert.deepEqual(media.point, { x: 115.9, y: -32 });

  upstreamFails = true;
  const failed = await deliver("upstream-failure", routeJob);
  assert.deepEqual(failed.explicitAcks, []);
  assert.equal(failed.retryMessages[0].msgId, "upstream-failure");

  // One malformed message must not stop the next valid message in a batch.
  upstreamFails = false;
  const mixed = await withinTimeout(
    worker.queue("howitt-jobs", [
      {
        id: "invalid",
        timestamp: new Date(),
        body: { version: "99" },
        attempts: 1,
      },
      { id: "valid", timestamp: new Date(), body: routeJob, attempts: 1 },
    ]),
  );
  assert.deepEqual(mixed.explicitAcks, ["valid"]);
  assert.equal(mixed.retryMessages[0].msgId, "invalid");

  const webhookJob = {
    version: "1",
    job: {
      Rwgps: {
        Webhook: {
          user_id: 81,
          item_type: "route",
          item_id: 314,
          item_user_id: 81,
          item_url: "https://ridewithgps.com/routes/314",
          action: "updated",
          collection: null,
        },
      },
    },
  };
  const webhook = await deliver("webhook", webhookJob);
  assert.deepEqual(webhook.explicitAcks, ["webhook"]);
  for (let i = 0; i < 100 && children.length === 0; i++) await Bun.sleep(20);
  assert.deepEqual(children, [routeJob]);
  // Missing producer binding forces a publication failure after successful
  // webhook processing. The parent must remain retryable, never acknowledged.
  const [jobsWorker, childrenWorker] = options.workers;
  assert(jobsWorker && childrenWorker);
  missingProducer = new Miniflare({
    ...options,
    workers: [
      {
        ...jobsWorker,
        config: {
          ...jobsWorker.config,
          env: {
            HYPERDRIVE: {
              type: "hyperdrive",
              id: "local",
              dev: { connectionString: database },
            },
          },
        },
      },
      childrenWorker,
    ],
  });
  const brokenProducer = await missingProducer.getWorker("jobs");
  const publicationFailure = await withinTimeout(
    brokenProducer.queue("howitt-jobs", [
      {
        id: "publication-failure",
        timestamp: new Date(),
        body: webhookJob,
        attempts: 1,
      },
    ]),
  );
  assert.deepEqual(publicationFailure.explicitAcks, []);
  assert.equal(
    publicationFailure.retryMessages[0].msgId,
    "publication-failure",
  );
  console.log(
    "PASS: Wasm queue → mocked RWGPS → local Hyperdrive/Postgres, credential rotation, duplicate delivery, per-message retry, webhook fan-out and publication failure",
  );
} finally {
  await missingProducer?.dispose();
  await mf.dispose();
  await sql.close();
}

async function withinTimeout<T>(operation: Promise<T>): Promise<T> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([
      operation,
      new Promise<never>((_, reject) => {
        timer = setTimeout(
          () => reject(new Error("Queue invocation timed out")),
          30_000,
        );
      }),
    ]);
  } finally {
    clearTimeout(timer);
  }
}
