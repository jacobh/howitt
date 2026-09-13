import { createHash } from "node:crypto";
import { copyFile, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

const metadata: {
  packages: { name: string; version: string; manifest_path: string }[];
} = JSON.parse(await Bun.stdin.text());
const dataset = metadata.packages.find(
  (pkg) => pkg.name === "tzf-dist" && pkg.version === "0.0.2026-c-tzb1",
);
if (!dataset)
  throw new Error("Expected locked tzf-dist 0.0.2026-c-tzb1 dataset");
const source = dirname(dataset.manifest_path);
const destination = "src/bin/howitt-web/build/assets/timezones";
await rm(destination, { recursive: true, force: true });
await mkdir(destination, { recursive: true });

const sourceFile = "lite.tzb";
const contents = await readFile(join(source, sourceFile));
const expected =
  "618d067b7746d5eca799c5f25d9da4013b095a94d9aecc6261a81015ba2d8f34";
if (createHash("sha256").update(contents).digest("hex") !== expected) {
  throw new Error(`Timezone dataset checksum mismatch: ${sourceFile}`);
}
await copyFile(join(source, sourceFile), join(destination, sourceFile));
await copyFile(join(source, "LICENSE_DATA"), join(destination, "LICENSE.txt"));
await writeFile(
  join(destination, "ATTRIBUTION.txt"),
  "Timezone data: tzf-dist 0.0.2026-c-tzb1 (ODbL-1.0) by ringsaturn.\n" +
    "https://github.com/ringsaturn/tzf-dist\n" +
    "Derived from timezone-boundary-builder and OpenStreetMap contributors.\n" +
    "https://github.com/evansiroky/timezone-boundary-builder\n" +
    "https://www.openstreetmap.org/copyright\n" +
    "Dataset copied unchanged. See LICENSE.txt.\n",
);
