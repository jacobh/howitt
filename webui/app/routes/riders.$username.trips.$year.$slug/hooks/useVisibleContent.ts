import { useMutativeReducer } from "use-mutative";
import { ContentBlockEvent } from "../components/ContentBlock";
import { match } from "ts-pattern";
import { useCallback } from "react";

type State = {
  routeIdVisibleContentBlocks: Map<string, Set<string>>;
  mediaIdVisibleContentBlocks: Map<string, Set<string>>;
  mediaIdHoveredContentBlocks: Map<string, Set<string>>;
};

const initialState: State = {
  routeIdVisibleContentBlocks: new Map(),
  mediaIdVisibleContentBlocks: new Map(),
  mediaIdHoveredContentBlocks: new Map(),
};

/**
 * Helper function to add a contentBlockId to a map of string sets.
 * Using Set ensures unique contentBlockIds per key.
 */
function addContentBlockIdToMap(
  map: Map<string, Set<string>>,
  key: string,
  contentBlockId: string,
): void {
  const existingSet = map.get(key) ?? new Set();
  existingSet.add(contentBlockId);
  map.set(key, existingSet);
}

/**
 * Helper function to remove a contentBlockId from a map of string sets.
 * If the set becomes empty after removal, the key is removed from the map.
 */
function removeContentBlockIdFromMap(
  map: Map<string, Set<string>>,
  key: string,
  contentBlockId: string,
): void {
  const existingSet = map.get(key);
  if (existingSet) {
    existingSet.delete(contentBlockId);
    if (existingSet.size === 0) {
      map.delete(key);
    }
  }
}

function reducer(
  draft: State,
  { contentBlockId, rideIds, mediaIds, eventType }: ContentBlockEvent,
): void {
  match(eventType)
    .with("visibleStart", () => {
      // Handle route IDs
      for (const rideId of rideIds) {
        addContentBlockIdToMap(
          draft.routeIdVisibleContentBlocks,
          rideId,
          contentBlockId,
        );
      }

      // Handle media IDs
      for (const mediaId of mediaIds) {
        addContentBlockIdToMap(
          draft.mediaIdVisibleContentBlocks,
          mediaId,
          contentBlockId,
        );
      }
    })
    .with("visibleEnd", () => {
      // Handle route IDs
      for (const rideId of rideIds) {
        removeContentBlockIdFromMap(
          draft.routeIdVisibleContentBlocks,
          rideId,
          contentBlockId,
        );
      }

      // Handle media IDs
      for (const mediaId of mediaIds) {
        removeContentBlockIdFromMap(
          draft.mediaIdVisibleContentBlocks,
          mediaId,
          contentBlockId,
        );
      }
    })
    .with("hoverStart", () => {
      // Handle media IDs for hover start
      for (const mediaId of mediaIds) {
        addContentBlockIdToMap(
          draft.mediaIdHoveredContentBlocks,
          mediaId,
          contentBlockId,
        );
      }
    })
    .with("hoverEnd", () => {
      // Handle media IDs for hover end
      for (const mediaId of mediaIds) {
        removeContentBlockIdFromMap(
          draft.mediaIdHoveredContentBlocks,
          mediaId,
          contentBlockId,
        );
      }
    })
    .with("click", () => {
      // Handle click events
      // This is a no-op for now, but we can add logic here if needed
    })
    .exhaustive();
}

export function useVisibleContent(): {
  onContentBlockEvent: (event: ContentBlockEvent) => void;
  visibleRouteIds: Set<string>;
  visibleMediaIds: Set<string>;
  hoveredMediaIds: Set<string>;
} {
  const [state, dispatch] = useMutativeReducer(reducer, initialState);

  const onContentBlockEvent = useCallback(
    (event: ContentBlockEvent): void => {
      dispatch(event);
    },
    [dispatch],
  );

  // Get all route IDs and media IDs that have visible content blocks
  const visibleRouteIds = new Set(state.routeIdVisibleContentBlocks.keys());
  const visibleMediaIds = new Set(state.mediaIdVisibleContentBlocks.keys());
  const hoveredMediaIds = new Set(state.mediaIdHoveredContentBlocks.keys());

  return {
    onContentBlockEvent,
    visibleRouteIds,
    visibleMediaIds,
    hoveredMediaIds,
  };
}
