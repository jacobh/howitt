import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

const appDirectory = fileURLToPath(new URL("../app", import.meta.url));
const forbiddenDateFormatters = [
  /\.toLocaleDateString\s*\(/,
  /\.toLocaleString\s*\(/,
  /Intl\.DateTimeFormat/,
];

function sourceFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = `${directory}/${entry.name}`;

    if (entry.isDirectory()) {
      return entry.name === "__generated__" ? [] : sourceFiles(path);
    }

    return /\.[jt]sx?$/.test(entry.name) ? [path] : [];
  });
}

test("user-facing dates use the shared unambiguous formatters", () => {
  const violations = sourceFiles(appDirectory).flatMap((path) => {
    const source = readFileSync(path, "utf8");

    return forbiddenDateFormatters
      .filter((pattern) => pattern.test(source))
      .map(
        (pattern) =>
          `${path.slice(appDirectory.length + 1)}: ${pattern.source}`,
      );
  });

  assert.deepEqual(
    violations,
    [],
    "Use a formatter from app/services/format.ts instead of locale-sensitive date APIs",
  );
});
