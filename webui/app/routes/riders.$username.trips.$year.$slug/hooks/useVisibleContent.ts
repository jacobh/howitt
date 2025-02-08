import { useMutativeReducer } from "use-mutative";
import { ContentBlockVisibilityEvent } from "../components/ContentBlock";

type State = {
  routeIdToContentBlocks: Map<string, string[]>;
  mediaIdToContentBlocks: Map<string, string[]>;
};

const initialState: State = {
  routeIdToContentBlocks: new Map(),
  mediaIdToContentBlocks: new Map(),
};

function reducer(draft: State, event: ContentBlockVisibilityEvent): void {
  const { contentBlockId, rideIds, mediaIds, isVisible } = event;

  // Handle route IDs
  for (const rideId of rideIds) {
    const existingBlocks = draft.routeIdToContentBlocks.get(rideId) ?? [];

    if (isVisible) {
      if (!existingBlocks.includes(contentBlockId)) {
        draft.routeIdToContentBlocks.set(rideId, [
          ...existingBlocks,
          contentBlockId,
        ]);
      }
    } else {
      const filteredBlocks = existingBlocks.filter(
        (id) => id !== contentBlockId,
      );
      if (filteredBlocks.length === 0) {
        draft.routeIdToContentBlocks.delete(rideId);
      } else {
        draft.routeIdToContentBlocks.set(rideId, filteredBlocks);
      }
    }
  }

  // Handle media IDs
  for (const mediaId of mediaIds) {
    const existingBlocks = draft.mediaIdToContentBlocks.get(mediaId) ?? [];

    if (isVisible) {
      if (!existingBlocks.includes(contentBlockId)) {
        draft.mediaIdToContentBlocks.set(mediaId, [
          ...existingBlocks,
          contentBlockId,
        ]);
      }
    } else {
      const filteredBlocks = existingBlocks.filter(
        (id) => id !== contentBlockId,
      );
      if (filteredBlocks.length === 0) {
        draft.mediaIdToContentBlocks.delete(mediaId);
      } else {
        draft.mediaIdToContentBlocks.set(mediaId, filteredBlocks);
      }
    }
  }
}

export function useVisibleContent(): {
  onContentBlockVisibilityChange: (event: ContentBlockVisibilityEvent) => void;
  visibleRouteIds: Set<string>;
  visibleMediaIds: Set<string>;
} {
  const [state, dispatch] = useMutativeReducer(reducer, initialState);

  const onContentBlockVisibilityChange = (
    event: ContentBlockVisibilityEvent,
  ): void => {
    dispatch(event);
  };

  // Get all route IDs and media IDs that have visible content blocks
  const visibleRouteIds = new Set(state.routeIdToContentBlocks.keys());
  const visibleMediaIds = new Set(state.mediaIdToContentBlocks.keys());

  console.log(visibleMediaIds);

  return {
    onContentBlockVisibilityChange,
    visibleRouteIds,
    visibleMediaIds,
  };
}
