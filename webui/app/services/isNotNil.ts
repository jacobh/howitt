import { isNil } from "lodash";

export function isNotNil<T>(x: T): x is Exclude<T, null | undefined> {
  return !isNil(x);
}
