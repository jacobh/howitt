import { match } from "ts-pattern";
import { css } from "@emotion/react";
import Markdown from "react-markdown";
import { RideItem } from "~/components/rides/RideItem";
import { Map as MapComponent } from "~/components/map";
import { isNotNil } from "~/services/isNotNil";
import { Track } from "~/components/map/types";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { useIntersectionObserver } from "usehooks-ts";
import { useMemo } from "react";

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
  margin: "16px 0",
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
  onVisibilityChange?: (event: ContentBlockVisibilityEvent) => void;
};

export interface ContentBlockVisibilityEvent {
  contentBlockId: string;
  rideIds: string[];
  mediaIds: string[];
  isVisible: boolean;
}

export function ContentBlock({
  block: blockFragment,
  rideIdRideMap,
  onVisibilityChange,
}: ContentBlockProps) {
  const block = useFragment(ContentBlockFragment, blockFragment);

  const contentBlockId = useMemo(
    () =>
      match(block)
        .with({ __typename: "Ride" }, (ride) => `ride-${ride.rideId}`)
        .with({ __typename: "Note" }, (note) => `note-${note.contentAt}`)
        .with({ __typename: "Media" }, (media) => `media-${media.mediaId}`)
        .exhaustive(),
    [block],
  );

  const rideIds = useMemo(
    () =>
      match(block)
        .with({ __typename: "Ride" }, (ride) => [ride.rideId])
        .with({ __typename: "Note" }, (note) =>
          note.ride ? [note.ride.id] : [],
        )
        .with({ __typename: "Media" }, (media) =>
          media.rides.map((ride) => ride.id),
        )
        .exhaustive(),
    [block],
  );

  const mediaIds = useMemo(
    () =>
      match(block)
        .with({ __typename: "Ride" }, () => [])
        .with({ __typename: "Note" }, () => [])
        .with({ __typename: "Media" }, (media) => [media.mediaId])
        .exhaustive(),
    [block],
  );

  const { ref } = useIntersectionObserver({
    threshold: 0.2,
    onChange: (isIntersecting) => {
      if (onVisibilityChange) {
        onVisibilityChange({
          contentBlockId,
          rideIds,
          mediaIds,
          isVisible: isIntersecting,
        });
      }
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
      <img src={media.imageSizes.fit1600.webpUrl} css={mediaStyles} alt="" />
    ))
    .exhaustive();

  return (
    <div ref={ref} key={contentBlockId}>
      {content}
    </div>
  );
}
