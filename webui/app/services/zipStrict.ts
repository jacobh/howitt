export function zipStrict<T, U>(items1: T[], items2: U[]): [T, U][] {
  if (items1.length !== items2.length) {
    throw new Error("items must have same length");
  }

  return items1.map((x, i) => [x, items2[i]]);
}
