import { useMutativeReducer } from "use-mutative";
import { ContentBlockVisibilityEvent } from "../components/ContentBlock";

type State = {
  routeIdToContentBlocks: Map<string, string[]>;
};

const initialState: State = {
  routeIdToContentBlocks: new Map(),
};

function reducer(draft: State, event: ContentBlockVisibilityEvent) {
  console.log(event);

  const { contentBlockId, rideIds, isVisible } = event;

  for (const rideId of rideIds) {
    const existingBlocks = draft.routeIdToContentBlocks.get(rideId) ?? [];

    if (isVisible) {
      // Add contentBlockId if not already present
      if (!existingBlocks.includes(contentBlockId)) {
        draft.routeIdToContentBlocks.set(rideId, [
          ...existingBlocks,
          contentBlockId,
        ]);
      }
    } else {
      // Remove contentBlockId if present
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
}

export function useVisibleRoutes() {
  const [state, dispatch] = useMutativeReducer(reducer, initialState);

  const onContentBlockVisibilityChange = (
    event: ContentBlockVisibilityEvent,
  ) => {
    dispatch(event);
  };

  // Get all route IDs that have visible content blocks
  const visibleRouteIds = new Set(state.routeIdToContentBlocks.keys());

  return {
    onContentBlockVisibilityChange,
    visibleRouteIds,
  };
}
