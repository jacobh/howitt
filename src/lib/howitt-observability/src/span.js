import { tracing } from "cloudflare:workers";
// worker-build 0.8.5 externalizes cloudflare:* but not node:* imports.
// Leave this built-in module for workerd to resolve, not its inner bundler.
const asyncHooksModule = "node:async_hooks";
const { AsyncLocalStorage } = await import(asyncHooksModule);

export function startSpan(name) {
  return tracing.startActiveSpan(name, (span) => {
    if (!span.isTraced) {
      span.end();
      return null;
    }
    return { span, run: AsyncLocalStorage.snapshot() };
  });
}

export function setSpanAttribute(state, key, value) {
  state.span.setAttribute(key, value);
}

export function captureContext() {
  return { run: AsyncLocalStorage.snapshot() };
}

// The borrowed Wasm callback is synchronous and must never be retained.
export function pollSpan(state, callback) {
  state.run(callback);
}

export function endSpan(state) {
  state.span.end();
}
