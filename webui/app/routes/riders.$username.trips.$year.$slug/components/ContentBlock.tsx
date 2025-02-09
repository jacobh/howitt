import { match } from "ts-pattern";
import { css } from "@emotion/react";
import Markdown from "react-markdown";
import { RideItem } from "~/components/rides/RideItem";
import { Map as MapComponent } from "~/components/map";
import { isNotNil } from "~/services/isNotNil";
import { Track } from "~/components/map/types";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { useIntersectionObserver } from "usehooks-ts";
import { useCallback, useMemo, useState } from "react";

const contentBlockStyles = css({
  position: "relative", // This ensures the overlay positions relative to this container
});

const overlayStyles = css({
  position: "absolute",
  top: "8px",
  right: "8px",
  backgroundColor: "rgba(0, 0, 0, 0.7)",
  color: "white",
  padding: "8px 12px",
  borderRadius: "4px",
  fontSize: "14px",
  pointerEvents: "none",
  opacity: 0,
  transition: "opacity 0.05s ease-in-out",
});

const rideItemStyles = css({
  margin: "24px 0",
});

const rideMapStyles = css({
  height: "450px",
  marginBottom: "24px",
});

const noteStyles = css({
  margin: "24px 0",

  "ul, ol": {
    marginLeft: "24px",
    marginTop: "12px",
    marginBottom: "12px",
  },

  ul: {
    listStyleType: "disc",
  },

  ol: {
    listStyleType: "decimal",
  },

  li: {
    marginBottom: "8px",

    "&:last-child": {
      marginBottom: 0,
    },
  },
});

const mediaStyles = css({
  width: "100%",
  height: "auto",
  borderRadius: "4px",
  margin: "8px 0",
});

const mediaCaptionStyles = css({
  color: "#464646",
  fontSize: "14px",
  margin: "8px 4px 24px",
});

const dividerStyles = css({
  margin: "32px 0",
  border: 0,
  borderTop: "1px solid #e5e7eb",
});

export const ContentBlockFragment = gql(`
  fragment contentBlock on TemporalContentBlock {
    __typename
    contentAt
    ... on Ride {
      rideId: id
      ...rideItem
    }
    ... on Media {
      mediaId: id
      capturedAt
      imageSizes {
        fit1600 {
          webpUrl
        }
      }
      rides {
        id
      }
    }
    ... on Note {
      text
      ride {
        id
      }
    }
  }
`);

type ContentBlockProps = {
  block: FragmentType<typeof ContentBlockFragment>;
  rideIdRideMap: Map<string, Track>;
  onEvent?: (event: ContentBlockEvent) => void;
};

export interface ContentBlockEvent {
  contentBlockId: string;
  rideIds: string[];
  mediaIds: string[];
  eventType:
    | "visibleStart"
    | "visibleEnd"
    | "hoverStart"
    | "hoverEnd"
    | "click";
  contentType: "Ride" | "Media" | "Note";
}

export function ContentBlock({
  block: blockFragment,
  rideIdRideMap,
  onEvent,
}: ContentBlockProps): React.ReactElement {
  const block = useFragment(ContentBlockFragment, blockFragment);
  const [isHovered, setIsHovered] = useState(false);

  const { contentBlockId, rideIds, mediaIds, contentType } = useMemo(
    () =>
      match(block)
        .with({ __typename: "Ride" }, (ride) => ({
          contentBlockId: ride.rideId,
          rideIds: [ride.rideId],
          mediaIds: [],
          contentType: "Ride" as const,
        }))
        .with({ __typename: "Note" }, (note) => ({
          contentBlockId: note.contentAt,
          rideIds: note.ride ? [note.ride.id] : [],
          mediaIds: [],
          contentType: "Note" as const,
        }))
        .with({ __typename: "Media" }, (media) => ({
          contentBlockId: media.mediaId,
          rideIds: media.rides.map((ride) => ride.id),
          mediaIds: [media.mediaId],
          contentType: "Media" as const,
        }))
        .exhaustive(),
    [block],
  );

  const { ref } = useIntersectionObserver({
    threshold: 0.2,
    onChange: (isIntersecting) => {
      onEvent?.({
        contentBlockId,
        rideIds,
        mediaIds,
        contentType,
        eventType: isIntersecting ? "visibleStart" : "visibleEnd",
      });
    },
  });

  const content = match(block)
    .with({ __typename: "Ride" }, (ride) => (
      <div css={rideItemStyles}>
        {/* <hr css={dividerStyles} /> */}
        <div css={rideMapStyles}>
          <MapComponent
            interactive={false}
            tracks={[rideIdRideMap.get(ride.rideId)].filter(isNotNil)}
            initialView={{
              type: "tracks",
              trackIds: [ride.rideId],
            }}
          />
        </div>
        <RideItem ride={ride} />
      </div>
    ))
    .with({ __typename: "Note" }, (note) => (
      <section css={noteStyles}>
        <Markdown>{note.text}</Markdown>
      </section>
    ))
    .with({ __typename: "Media" }, (media) => (
      <div>
        <img src={media.imageSizes.fit1600.webpUrl} css={mediaStyles} alt="" />
        <div css={mediaCaptionStyles}>
          {new Date(media.capturedAt).toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
          })}
        </div>
      </div>
    ))
    .exhaustive();

  const handleMouseEnter = useCallback((): void => {
    setIsHovered(true);
    onEvent?.({
      contentBlockId,
      rideIds,
      mediaIds,
      contentType,
      eventType: "hoverStart",
    });
  }, [contentBlockId, rideIds, mediaIds, contentType, onEvent]);

  const handleMouseLeave = useCallback((): void => {
    setIsHovered(false);
    onEvent?.({
      contentBlockId,
      rideIds,
      mediaIds,
      contentType,
      eventType: "hoverEnd",
    });
  }, [contentBlockId, rideIds, mediaIds, contentType, onEvent]);

  const handleClick = useCallback((): void => {
    onEvent?.({
      contentBlockId,
      rideIds,
      mediaIds,
      contentType,
      eventType: "click",
    });
  }, [contentBlockId, rideIds, mediaIds, contentType, onEvent]);

  const handleKeyDown = useCallback(
    (event: React.KeyboardEvent): void => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        onEvent?.({
          contentBlockId,
          rideIds,
          mediaIds,
          contentType,
          eventType: "click",
        });
      }
    },
    [contentBlockId, rideIds, mediaIds, contentType, onEvent],
  );

  return (
    <div
      ref={ref}
      key={contentBlockId}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      role="button"
      tabIndex={0}
      css={contentBlockStyles}
    >
      {content}
      <div css={overlayStyles} style={{ opacity: isHovered ? 1 : 0 }}>
        Click to see on map
      </div>
    </div>
  );
}
