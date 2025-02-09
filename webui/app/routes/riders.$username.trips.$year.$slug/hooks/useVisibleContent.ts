import { useMutativeReducer } from "use-mutative";
import { ContentBlockEvent } from "../components/ContentBlock";
import { match } from "ts-pattern";
import { useCallback } from "react";

type State = {
  routeIdToContentBlocks: Map<string, string[]>;
  mediaIdToContentBlocks: Map<string, string[]>;
};

const initialState: State = {
  routeIdToContentBlocks: new Map(),
  mediaIdToContentBlocks: new Map(),
};

function reducer(draft: State, event: ContentBlockEvent): void {
  const { contentBlockId, rideIds, mediaIds, eventType } = event;

  // Handle both route and media IDs based on eventType
  match(eventType)
    .with("visibleStart", () => {
      // Handle route IDs
      for (const rideId of rideIds) {
        const existingBlocks = draft.routeIdToContentBlocks.get(rideId) ?? [];
        if (!existingBlocks.includes(contentBlockId)) {
          draft.routeIdToContentBlocks.set(rideId, [
            ...existingBlocks,
            contentBlockId,
          ]);
        }
      }

      // Handle media IDs
      for (const mediaId of mediaIds) {
        const existingBlocks = draft.mediaIdToContentBlocks.get(mediaId) ?? [];
        if (!existingBlocks.includes(contentBlockId)) {
          draft.mediaIdToContentBlocks.set(mediaId, [
            ...existingBlocks,
            contentBlockId,
          ]);
        }
      }
    })
    .with("visibleEnd", () => {
      // Handle route IDs
      for (const rideId of rideIds) {
        const existingBlocks = draft.routeIdToContentBlocks.get(rideId) ?? [];
        const filteredBlocks = existingBlocks.filter(
          (id) => id !== contentBlockId,
        );
        if (filteredBlocks.length === 0) {
          draft.routeIdToContentBlocks.delete(rideId);
        } else {
          draft.routeIdToContentBlocks.set(rideId, filteredBlocks);
        }
      }

      // Handle media IDs
      for (const mediaId of mediaIds) {
        const existingBlocks = draft.mediaIdToContentBlocks.get(mediaId) ?? [];
        const filteredBlocks = existingBlocks.filter(
          (id) => id !== contentBlockId,
        );
        if (filteredBlocks.length === 0) {
          draft.mediaIdToContentBlocks.delete(mediaId);
        } else {
          draft.mediaIdToContentBlocks.set(mediaId, filteredBlocks);
        }
      }
    })
    .exhaustive();
}

export function useVisibleContent(): {
  onContentBlockEvent: (event: ContentBlockEvent) => void;
  visibleRouteIds: Set<string>;
  visibleMediaIds: Set<string>;
} {
  const [state, dispatch] = useMutativeReducer(reducer, initialState);

  const onContentBlockEvent = useCallback(
    (event: ContentBlockEvent): void => {
      dispatch(event);
    },
    [dispatch],
  );

  // Get all route IDs and media IDs that have visible content blocks
  const visibleRouteIds = new Set(state.routeIdToContentBlocks.keys());
  const visibleMediaIds = new Set(state.mediaIdToContentBlocks.keys());

  console.log(visibleMediaIds);

  return {
    onContentBlockEvent,
    visibleRouteIds,
    visibleMediaIds,
  };
}
