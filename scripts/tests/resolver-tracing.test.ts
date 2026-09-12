import { AsyncLocalStorage } from "node:async_hooks";
import { beforeEach, expect, mock, test } from "bun:test";

interface CapturedSpan {
  name: string;
  parent: CapturedSpan | undefined;
  attributes: Record<string, string>;
  isTraced: boolean;
  ends: number;
  setAttribute(key: string, value: string): void;
  end(): void;
}

const active = new AsyncLocalStorage<CapturedSpan>();
const spans: CapturedSpan[] = [];
let sampled = true;
mock.module("cloudflare:workers", () => ({
  tracing: {
    startActiveSpan<T>(name: string, callback: (span: CapturedSpan) => T): T {
      const span: CapturedSpan = {
        name,
        parent: active.getStore(),
        attributes: {},
        isTraced: sampled,
        ends: 0,
        setAttribute(key, value) {
          this.attributes[key] = value;
        },
        end() {
          this.ends += 1;
          this.isTraced = false;
        },
      };
      spans.push(span);
      return active.run(span, () => callback(span));
    },
  },
}));

const {
  startSpan,
  captureContext,
  pollSpan: pollResolverSpan,
  setSpanAttribute,
  endSpan,
} = await import("../../src/lib/howitt-observability/src/span.js");
function startResolverSpan(
  parentType: string,
  field: string,
  returnType: string,
) {
  const state = startSpan(`graphql.resolve ${parentType}.${field}`);
  if (state !== null) {
    setSpanAttribute(state, "graphql.parent_type", parentType);
    setSpanAttribute(state, "graphql.field.name", field);
    setSpanAttribute(state, "graphql.return_type", returnType);
  }
  return state;
}
function endResolverSpan(state: ReturnType<typeof startSpan>, outcome: string) {
  setSpanAttribute(state, "graphql.outcome", outcome);
  endSpan(state);
}

beforeEach(() => {
  spans.length = 0;
  sampled = true;
});

test("restores each resolver context on every poll without leaking into siblings", async () => {
  const a = startResolverSpan("Query", "a", "Item!");
  const b = startResolverSpan("Query", "b", "Item!");
  expect(active.getStore()).toBeUndefined();

  pollResolverSpan(a, () => {
    expect(active.getStore()?.name).toBe("graphql.resolve Query.a");
    const child = startResolverSpan("Item", "value", "Int!");
    endResolverSpan(child, "ok");
  });
  await Promise.resolve();
  pollResolverSpan(b, () => {
    expect(active.getStore()?.name).toBe("graphql.resolve Query.b");
    const child = startResolverSpan("Item", "value", "Int!");
    endResolverSpan(child, "error");
  });
  await Promise.resolve();
  pollResolverSpan(a, () => {
    expect(active.getStore()?.name).toBe("graphql.resolve Query.a");
    const child = startResolverSpan("Item", "afterAwait", "Int!");
    endResolverSpan(child, "ok");
  });
  expect(active.getStore()).toBeUndefined();
  endResolverSpan(a, "ok");
  endResolverSpan(b, "cancelled");

  expect(spans.map((span) => span.parent?.name)).toEqual([
    undefined,
    undefined,
    "graphql.resolve Query.a",
    "graphql.resolve Query.b",
    "graphql.resolve Query.a",
  ]);
  expect(spans.every((span) => span.ends === 1)).toBe(true);
  expect(spans[0]?.attributes).toEqual({
    "graphql.parent_type": "Query",
    "graphql.field.name": "a",
    "graphql.return_type": "Item!",
    "graphql.outcome": "ok",
  });
  expect(spans[1]?.attributes["graphql.outcome"]).toBe("cancelled");
  expect(spans[3]?.attributes["graphql.outcome"]).toBe("error");
});

test("spawned batches retain their caller context when another task wakes the queue", () => {
  const resolver = startSpan("graphql.resolve Route.points");
  let task: ReturnType<typeof captureContext> | undefined;
  pollResolverSpan(resolver, () => {
    task = captureContext();
  });
  if (task === undefined) throw new Error("Task context was not captured");
  const captured = task;
  const unrelated = startSpan("repo.User.get");
  pollResolverSpan(unrelated, () => {
    pollResolverSpan(captured, () => {
      const batch = startSpan("repo.RoutePoints.filter_models");
      endSpan(batch);
    });
  });
  endSpan(resolver);
  endSpan(unrelated);
  expect(spans[2]?.parent?.name).toBe("graphql.resolve Route.points");
  expect(active.getStore()).toBeUndefined();
});

test("does not retain a context snapshot when the invocation is not sampled", () => {
  sampled = false;
  expect(startResolverSpan("Query", "a", "Item!")).toBeNull();
  expect(spans[0]?.ends).toBe(1);
  expect(spans[0]?.attributes).toEqual({});
  expect(active.getStore()).toBeUndefined();
});
