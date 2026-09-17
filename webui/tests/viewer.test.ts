import assert from "node:assert/strict";
import { test } from "vitest";

import { resolveViewerState } from "~/components/layout/Viewer";

test("viewer state distinguishes an unresolved query from an anonymous viewer", () => {
  assert.deepEqual(resolveViewerState(undefined), { status: "loading" });
  assert.deepEqual(resolveViewerState({ viewer: null }), {
    status: "anonymous",
  });
});

test("viewer state retains authenticated viewer details", () => {
  const viewer = { id: "viewer-1", profile: { username: "alice" } };

  assert.deepEqual(resolveViewerState({ viewer }), {
    status: "authenticated",
    viewer,
  });
});
