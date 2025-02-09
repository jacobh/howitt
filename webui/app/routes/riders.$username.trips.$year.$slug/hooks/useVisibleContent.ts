import { useMutativeReducer } from "use-mutative";
import { ContentBlockEvent } from "../components/ContentBlock";
import { match } from "ts-pattern";
import { useCallback } from "react";

type State = {
  routeIdVisibleContentBlocks: Map<string, string[]>;
  mediaIdVisibleContentBlocks: Map<string, string[]>;
  mediaIdHoveredContentBlocks: Map<string, string[]>;
};

const initialState: State = {
  routeIdVisibleContentBlocks: new Map(),
  mediaIdVisibleContentBlocks: new Map(),
  mediaIdHoveredContentBlocks: new Map(),
};

function reducer(draft: State, event: ContentBlockEvent): void {
  const { contentBlockId, rideIds, mediaIds, eventType } = event;

  match(eventType)
    .with("visibleStart", () => {
      // Handle route IDs
      for (const rideId of rideIds) {
        const existingBlocks =
          draft.routeIdVisibleContentBlocks.get(rideId) ?? [];
        if (!existingBlocks.includes(contentBlockId)) {
          draft.routeIdVisibleContentBlocks.set(rideId, [
            ...existingBlocks,
            contentBlockId,
          ]);
        }
      }

      // Handle media IDs
      for (const mediaId of mediaIds) {
        const existingBlocks =
          draft.mediaIdVisibleContentBlocks.get(mediaId) ?? [];
        if (!existingBlocks.includes(contentBlockId)) {
          draft.mediaIdVisibleContentBlocks.set(mediaId, [
            ...existingBlocks,
            contentBlockId,
          ]);
        }
      }
    })
    .with("visibleEnd", () => {
      // Handle route IDs
      for (const rideId of rideIds) {
        const existingBlocks =
          draft.routeIdVisibleContentBlocks.get(rideId) ?? [];
        const filteredBlocks = existingBlocks.filter(
          (id) => id !== contentBlockId,
        );
        if (filteredBlocks.length === 0) {
          draft.routeIdVisibleContentBlocks.delete(rideId);
        } else {
          draft.routeIdVisibleContentBlocks.set(rideId, filteredBlocks);
        }
      }

      // Handle media IDs
      for (const mediaId of mediaIds) {
        const existingBlocks =
          draft.mediaIdVisibleContentBlocks.get(mediaId) ?? [];
        const filteredBlocks = existingBlocks.filter(
          (id) => id !== contentBlockId,
        );
        if (filteredBlocks.length === 0) {
          draft.mediaIdVisibleContentBlocks.delete(mediaId);
        } else {
          draft.mediaIdVisibleContentBlocks.set(mediaId, filteredBlocks);
        }
      }
    })
    .with("hoverStart", () => {
      // Handle media IDs for hover start
      for (const mediaId of mediaIds) {
        const existingBlocks =
          draft.mediaIdHoveredContentBlocks.get(mediaId) ?? [];
        if (!existingBlocks.includes(contentBlockId)) {
          draft.mediaIdHoveredContentBlocks.set(mediaId, [
            ...existingBlocks,
            contentBlockId,
          ]);
        }
      }
    })
    .with("hoverEnd", () => {
      // Handle media IDs for hover end
      for (const mediaId of mediaIds) {
        const existingBlocks =
          draft.mediaIdHoveredContentBlocks.get(mediaId) ?? [];
        const filteredBlocks = existingBlocks.filter(
          (id) => id !== contentBlockId,
        );
        if (filteredBlocks.length === 0) {
          draft.mediaIdHoveredContentBlocks.delete(mediaId);
        } else {
          draft.mediaIdHoveredContentBlocks.set(mediaId, filteredBlocks);
        }
      }
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
