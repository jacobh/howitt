import { isNil } from "lodash";

export function isNotNil<T>(x: T): x is Exclude<T, null | undefined> {
  return !isNil(x);
}

export function unwrapNil<T>(x: T): Exclude<T, null | undefined> {
  if (isNotNil(x)) {
    return x;
  }
  throw new Error("Expected value to be non-nil");
}
