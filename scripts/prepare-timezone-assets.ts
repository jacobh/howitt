import { createHash } from "node:crypto";
import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

const metadata: { packages: { name: string; version: string; manifest_path: string }[] } =
  JSON.parse(await Bun.stdin.text());
const dataset = metadata.packages.find(
  (pkg) => pkg.name === "tzf-rel" && pkg.version === "0.0.2025-a",
);
if (!dataset) throw new Error("Expected locked tzf-rel 0.0.2025-a dataset");
const source = dirname(dataset.manifest_path);
const destination = "src/bin/howitt-web/build/assets/timezones";
await mkdir(destination, { recursive: true });

const files: [string, string][] = [
  ["combined-with-oceans.reduce.pb", "b833c1db6215ade36dc08ad1aa9275ba51db03e450a8a54ee05f80d0ccd68819"],
  ["combined-with-oceans.reduce.preindex.pb", "7b69582ec5edfcf66d62a8e435e30ed801837b175d5bb4ab6c364f6c73e1ccb6"],
];
for (const [file, expected] of files) {
  const contents = await readFile(join(source, file));
  if (createHash("sha256").update(contents).digest("hex") !== expected) {
    throw new Error(`Timezone dataset checksum mismatch: ${file}`);
  }
  await copyFile(join(source, file), join(destination, file));
}
await copyFile(join(source, "LICENSE"), join(destination, "LICENSE.txt"));
await writeFile(join(destination, "ATTRIBUTION.txt"),
  "Timezone data: tzf-rel 0.0.2025-a (ODbL-1.0) by ringsaturn.\n" +
  "https://github.com/ringsaturn/tzf-rel\n" +
  "Derived from timezone-boundary-builder and OpenStreetMap contributors.\n" +
  "https://github.com/evansiroky/timezone-boundary-builder\n" +
  "https://www.openstreetmap.org/copyright\n" +
  "Dataset copied unchanged. See LICENSE.txt.\n");
