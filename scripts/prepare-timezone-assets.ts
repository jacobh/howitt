import { createHash } from "node:crypto";
import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

const metadata: {
  packages: { name: string; version: string; manifest_path: string }[];
} = JSON.parse(await Bun.stdin.text());
const dataset = metadata.packages.find(
  (pkg) => pkg.name === "tzf-rel" && pkg.version === "0.0.2025-c",
);
if (!dataset) throw new Error("Expected locked tzf-rel 0.0.2025-c dataset");
const source = dirname(dataset.manifest_path);
const destination = "src/bin/howitt-web/build/assets/timezones";
await mkdir(destination, { recursive: true });

const files: [string, string, string][] = [
  [
    "combined-with-oceans.reduce.bin",
    "combined-with-oceans.reduce.pb",
    "65c63c5b2670b64abce4a0286c5ad3d73f131ea640ef3a89b8a4d822dd8ba5b3",
  ],
  [
    "combined-with-oceans.reduce.preindex.bin",
    "combined-with-oceans.reduce.preindex.pb",
    "a01ead834abe57193627dbe06cb480dabe32be6ac665770a1e1e634fe913cd92",
  ],
];
for (const [sourceFile, destinationFile, expected] of files) {
  const contents = await readFile(join(source, sourceFile));
  if (createHash("sha256").update(contents).digest("hex") !== expected) {
    throw new Error(`Timezone dataset checksum mismatch: ${sourceFile}`);
  }
  await copyFile(join(source, sourceFile), join(destination, destinationFile));
}
await copyFile(join(source, "LICENSE"), join(destination, "LICENSE.txt"));
await writeFile(
  join(destination, "ATTRIBUTION.txt"),
  "Timezone data: tzf-rel 0.0.2025-c (ODbL-1.0) by ringsaturn.\n" +
    "https://github.com/ringsaturn/tzf-rel\n" +
    "Derived from timezone-boundary-builder and OpenStreetMap contributors.\n" +
    "https://github.com/evansiroky/timezone-boundary-builder\n" +
    "https://www.openstreetmap.org/copyright\n" +
    "Dataset copied unchanged. See LICENSE.txt.\n",
);
