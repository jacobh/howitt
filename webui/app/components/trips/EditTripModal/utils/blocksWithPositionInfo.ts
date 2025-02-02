export type BlockWithPositionInfo<T> =
  | {
      position: "first";
      block: T;
      nextBlock: T | undefined;
      prevBlock: undefined;
      idx: 0;
    }
  | {
      position: "middle";
      block: T;
      prevBlock: T | undefined;
      nextBlock: T | undefined;
      idx: number;
    }
  | {
      position: "last";
      block: T;
      prevBlock: T;
      nextBlock: undefined;
      idx: number;
    }
  | {
      position: "only";
      block: T;
      prevBlock: undefined;
      nextBlock: undefined;
      idx: 0;
    };

export function blocksWithPositionInfo<T>(
  blocks: T[],
): BlockWithPositionInfo<T>[] {
  if (blocks.length === 1) {
    return [
      {
        position: "only" as const,
        block: blocks[0],
        prevBlock: undefined,
        nextBlock: undefined,
        idx: 0 as const,
      },
    ];
  }

  return blocks.map((block, idx) => {
    if (idx === 0) {
      return {
        position: "first" as const,
        block,
        prevBlock: undefined,
        nextBlock: blocks[1],
        idx: 0 as const,
      };
    }

    if (idx === blocks.length - 1) {
      return {
        position: "last" as const,
        block,
        prevBlock: blocks[idx - 1],
        nextBlock: undefined,
        idx,
      };
    }

    return {
      position: "middle" as const,
      block,
      prevBlock: blocks[idx - 1],
      nextBlock: blocks[idx + 1],
      idx,
    };
  });
}
