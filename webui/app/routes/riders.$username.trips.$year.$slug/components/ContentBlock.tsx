import { match } from "ts-pattern";
import { css } from "@emotion/react";
import Markdown from "react-markdown";
import { RideItem } from "~/components/rides/RideItem";
import { Map as MapComponent } from "~/components/map";
import { isNotNil } from "~/services/isNotNil";
import { Track } from "~/components/map/types";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { useIntersectionObserver } from "usehooks-ts";
import { useCallback, useMemo } from "react";

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
  eventType: "visibleStart" | "visibleEnd" | "hoverStart" | "hoverEnd";
}

export function ContentBlock({
  block: blockFragment,
  rideIdRideMap,
  onEvent,
}: ContentBlockProps): React.ReactElement {
  const block = useFragment(ContentBlockFragment, blockFragment);

  const { contentBlockId, rideIds, mediaIds } = useMemo(
    () =>
      match(block)
        .with({ __typename: "Ride" }, (ride) => ({
          contentBlockId: `ride-${ride.rideId}`,
          rideIds: [ride.rideId],
          mediaIds: [],
        }))
        .with({ __typename: "Note" }, (note) => ({
          contentBlockId: `note-${note.contentAt}`,
          rideIds: note.ride ? [note.ride.id] : [],
          mediaIds: [],
        }))
        .with({ __typename: "Media" }, (media) => ({
          contentBlockId: `media-${media.mediaId}`,
          rideIds: media.rides.map((ride) => ride.id),
          mediaIds: [media.mediaId],
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
        eventType: isIntersecting ? "visibleStart" : "visibleEnd",
      });
    },
  });

  const content = match(block)
    .with({ __typename: "Ride" }, (ride) => (
      <div css={rideItemStyles}>
        <hr css={dividerStyles} />
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
    onEvent?.({
      contentBlockId,
      rideIds,
      mediaIds,
      eventType: "hoverStart",
    });
  }, [contentBlockId, rideIds, mediaIds, onEvent]);

  const handleMouseLeave = useCallback((): void => {
    onEvent?.({
      contentBlockId,
      rideIds,
      mediaIds,
      eventType: "hoverEnd",
    });
  }, [contentBlockId, rideIds, mediaIds, onEvent]);

  return (
    <div
      ref={ref}
      key={contentBlockId}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {content}
    </div>
  );
}
