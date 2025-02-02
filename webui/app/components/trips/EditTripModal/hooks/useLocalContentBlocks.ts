import { useCallback } from "react";
import { useMutative } from "use-mutative";
import { match, P } from "ts-pattern";
import { Temporal } from "@js-temporal/polyfill";
import type { TemporalContentBlockValue } from "../index";

interface UseLocalContentBlocksReturn {
  localContentBlocks: TemporalContentBlockValue[];
  onCreateNote: (index: number | "start" | "end") => void;
  onUpdateNote: (index: number, text: string) => void;
  onDeleteNote: (index: number) => void;
}

export function useLocalContentBlocks(
  initialContentBlocks: TemporalContentBlockValue[],
): UseLocalContentBlocksReturn {
  const [localContentBlocks, setLocalContentBlocks] =
    useMutative(initialContentBlocks);

  const onCreateNote = useCallback(
    (index: number | "start" | "end") => {
      setLocalContentBlocks((draft) => {
        match(index)
          .with("start", () => {
            const firstBlock = draft.at(0);
            const firstTimestamp = firstBlock
              ? Temporal.Instant.from(firstBlock.contentAt)
              : Temporal.Now.instant();
            const newTimestamp = firstTimestamp
              .subtract({ hours: 1 })
              .toString();

            draft.unshift({
              __typename: "Note" as const,
              contentAt: newTimestamp,
              text: "",
            });
          })
          .with("end", () => {
            const lastBlock = draft.at(-1);
            const lastTimestamp = lastBlock
              ? Temporal.Instant.from(lastBlock.contentAt)
              : Temporal.Now.instant();
            const newTimestamp = lastTimestamp.add({ hours: 1 }).toString();

            draft.push({
              __typename: "Note" as const,
              contentAt: newTimestamp,
              text: "",
            });
          })
          .with(P.number, (i) => {
            const currentBlock = draft[i];
            const nextBlock = draft.at(i + 1);

            const currentInstant = Temporal.Instant.from(
              currentBlock.contentAt,
            );
            const nextInstant = nextBlock
              ? Temporal.Instant.from(nextBlock.contentAt)
              : Temporal.Now.instant();

            const diffSeconds = nextInstant
              .since(currentInstant)
              .total("seconds");
            const newTimestamp = currentInstant
              .add({ seconds: Math.floor(diffSeconds / 2) })
              .toString();

            draft.splice(i + 1, 0, {
              __typename: "Note" as const,
              contentAt: newTimestamp,
              text: "",
            });
          })
          .exhaustive();
      });
    },
    [setLocalContentBlocks],
  );

  const onUpdateNote = useCallback(
    (index: number, text: string) => {
      setLocalContentBlocks((draft) => {
        const note = draft[index];
        if (note.__typename === "Note") {
          note.text = text;
        }
      });
    },
    [setLocalContentBlocks],
  );

  const onDeleteNote = useCallback(
    (index: number) => {
      setLocalContentBlocks((draft) => {
        draft.splice(index, 1);
      });
    },
    [setLocalContentBlocks],
  );

  return {
    localContentBlocks,
    onCreateNote,
    onUpdateNote,
    onDeleteNote,
  };
}
