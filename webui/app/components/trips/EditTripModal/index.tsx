import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { css } from "@emotion/react";
import { useCallback, useState } from "react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { Modal } from "../../Modal";
import { MediaTable } from "./components/MediaTable";
import { MediaDropzone } from "./components/MediaDropzone";
import * as Tabs from "@radix-ui/react-tabs";
import { match, P } from "ts-pattern";
import { isNotNil } from "~/services/isNotNil";
import { Temporal } from "@js-temporal/polyfill";
import { ResultOf } from "@graphql-typed-document-node/core";
import { blocksWithPositionInfo } from "./utils/blocksWithPositionInfo";

export const EditTripFragment = gql(`
    fragment editTrip on Trip {
    id
    name 
    description
    temporalContentBlocks {
      __typename
      contentAt
      ... on Note {
        text
      }
      ... on Media {
        mediaId: id
        imageSizes {
          fit1200 {
            webpUrl
          }
        }
      }
      ... on Ride {
        rideId: id
        name
      }
    }
    media {
      id
      path
      createdAt
      imageSizes {
        fill600 {
          webpUrl
        }
      }
    }
  }
`);

const UpdateTripMutation = gql(`
  mutation UpdateTrip($input: UpdateTripInput!) {
    updateTrip(input: $input) {
      trip {
        id
        name
        description
      }
    }
  }
`);

const RemoveTripMediaMutation = gql(`
  mutation RemoveTripMedia($input: RemoveTripMediaInput!) {
    removeTripMedia(input: $input) {
      trip {
        id
      }
    }
  }
`);

interface Props {
  trip: FragmentType<typeof EditTripFragment>;
  isOpen: boolean;
  refetch: () => void;
  onClose: () => void;
}

const formStyles = css`
  display: flex;
  flex-direction: column;
  gap: 1rem;
`;

const formFieldStyles = css`
  display: grid;
  grid-template-columns: minmax(75px, 6vw) 1fr;
  gap: 1rem;
  align-items: start;

  label {
    padding-top: 0.5rem;
  }
`;

const inputStyles = css`
  padding: 0.5rem;
  width: 100%;
  border: 1px solid #ccc;
`;

const buttonGroupStyles = css`
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1rem;
`;

const contentBlockContainerStyles = css`
  display: flex;
  flex-direction: column;
  gap: "1rem";
`;

const contentBlockStyles = css`
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
`;

const contentMetaStyles = css`
  color: #666;
  font-size: 0.9em;
`;

const mediaImageStyles = css`
  max-width: 600px;
  border-radius: 4px;
`;

const rideBlockStyles = css`
  padding: 0.5rem;
  background-color: #f5f5f5;
  border-radius: 4px;
`;

const addNoteButtonStyles = css`
  border: 1px dashed #ccc;
  padding: 0.5rem;
  text-align: center;
  cursor: pointer;
  color: #666;
  margin: 0.5rem 20px;

  &:hover {
    background-color: #f5f5f5;
  }
`;

const noteContainerStyles = css`
  display: flex;
  gap: 0.5rem;
`;

const deleteNoteButtonStyles = css`
  background: none;
  border: none;
  color: #666;
  cursor: pointer;
  padding: 0.25rem;

  &:hover {
    color: #ff4444;
  }
`;

const tabsRootStyles = css`
  display: flex;
  flex-direction: column;
  width: 100%;
`;

const tabsListStyles = css`
  display: flex;
  border-bottom: 1px solid #ccc;
  margin-bottom: 1rem;
`;

const tabTriggerStyles = css`
  padding: 0.5rem 1rem;
  border: none;
  background: none;
  cursor: pointer;

  &[data-state="active"] {
    border-bottom: 2px solid #000;
  }

  &:hover {
    background-color: #f5f5f5;
  }
`;

const tabContentStyles = css`
  &[data-state="inactive"] {
    display: none;
  }
`;

export function EditTripModal({
  trip: tripFragment,
  isOpen,
  onClose,
  refetch,
}: Props): React.ReactElement {
  const trip = useFragment(EditTripFragment, tripFragment);
  const [localContentBlocks, setLocalContentBlocks] = useState(
    trip.temporalContentBlocks,
  );
  const [uploading, setUploading] = useState(false);
  const [name, setName] = useState(trip.name);
  const [description, setDescription] = useState(trip.description ?? "");

  const [updateTrip, { loading }] = useMutation(UpdateTripMutation, {
    onCompleted: () => {
      refetch();
      onClose();
    },
  });

  const handleAddNote = useCallback(
    (index: number | "start" | "end") => {
      const updatedBlocks = match(index)
        .with("start", () => {
          const firstBlock = localContentBlocks.at(0);
          const firstTimestamp = firstBlock
            ? Temporal.Instant.from(firstBlock.contentAt)
            : Temporal.Now.instant();
          // Subtract 1 hour from the timestamp
          const newTimestamp = firstTimestamp.subtract({ hours: 1 }).toString();

          return [
            {
              __typename: "Note" as const,
              contentAt: newTimestamp,
              text: "",
            },
            ...localContentBlocks,
          ];
        })
        .with("end", () => {
          const lastBlock = localContentBlocks.at(-1);
          const lastTimestamp = lastBlock
            ? Temporal.Instant.from(lastBlock.contentAt)
            : Temporal.Now.instant();
          // Add 1 hour to the timestamp
          const newTimestamp = lastTimestamp.add({ hours: 1 }).toString();

          return [
            ...localContentBlocks,
            {
              __typename: "Note" as const,
              contentAt: newTimestamp,
              text: "",
            },
          ];
        })
        .with(P.number, (i) => {
          const currentBlock = localContentBlocks[i];
          const nextBlock = localContentBlocks.at(i + 1);

          const currentInstant = Temporal.Instant.from(currentBlock.contentAt);
          const nextInstant = nextBlock
            ? Temporal.Instant.from(nextBlock.contentAt)
            : Temporal.Now.instant();

          // Calculate the midpoint between timestamps
          const diffSeconds = nextInstant
            .since(currentInstant)
            .total("seconds");
          const newTimestamp = currentInstant
            .add({ seconds: Math.floor(diffSeconds / 2) })
            .toString();

          const newBlocks = [...localContentBlocks];
          newBlocks.splice(i + 1, 0, {
            __typename: "Note" as const,
            contentAt: newTimestamp,
            text: "",
          });
          return newBlocks;
        })
        .exhaustive();

      setLocalContentBlocks(updatedBlocks);
    },
    [localContentBlocks],
  );

  const handleDeleteNote = useCallback(
    (index: number) => {
      const updatedBlocks = [...localContentBlocks];
      updatedBlocks.splice(index, 1);
      setLocalContentBlocks(updatedBlocks);
    },
    [localContentBlocks],
  );

  const [removeMedia, { loading: removingMedia }] = useMutation(
    RemoveTripMediaMutation,
    {
      onCompleted: () => {
        refetch();
      },
    },
  );

  const handleRemoveMedia = useCallback(
    (mediaId: string): void => {
      removeMedia({
        variables: {
          input: {
            tripId: trip.id,
            mediaIds: [mediaId],
          },
        },
      });
    },
    [removeMedia, trip.id],
  );

  const handleSubmit = useCallback(
    (e: React.FormEvent): void => {
      e.preventDefault();

      // Extract notes from localContentBlocks
      const notes = localContentBlocks
        .map((block) =>
          match(block)
            .with({ __typename: "Note" }, (note) => ({
              timestamp: note.contentAt,
              text: note.text,
            }))
            .otherwise(() => null),
        )
        .filter(isNotNil);

      updateTrip({
        variables: {
          input: {
            tripId: trip.id,
            name,
            description: description || null,
            notes,
          },
        },
      });
    },
    [trip.id, name, description, localContentBlocks, updateTrip],
  );

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <form onSubmit={handleSubmit} css={formStyles}>
        <Tabs.Root defaultValue="trip" css={tabsRootStyles}>
          <Tabs.List css={tabsListStyles}>
            <Tabs.Trigger value="trip" css={tabTriggerStyles}>
              Trip
            </Tabs.Trigger>
            <Tabs.Trigger value="content" css={tabTriggerStyles}>
              Content
            </Tabs.Trigger>
            <Tabs.Trigger value="media" css={tabTriggerStyles}>
              Media
            </Tabs.Trigger>
          </Tabs.List>

          <Tabs.Content value="trip" css={tabContentStyles}>
            <div css={formFieldStyles}>
              <label htmlFor="name">Name</label>
              <input
                css={inputStyles}
                id="name"
                type="text"
                value={name}
                onChange={(e): void => setName(e.target.value)}
                autoComplete="off"
                required
              />
            </div>

            <div css={formFieldStyles}>
              <label htmlFor="description">Description</label>
              <textarea
                css={inputStyles}
                id="description"
                value={description}
                onChange={(e): void => setDescription(e.target.value)}
                rows={4}
              />
            </div>
          </Tabs.Content>

          <Tabs.Content value="content" css={tabContentStyles}>
            <h2>Content</h2>
            <div css={contentBlockContainerStyles}>
              {localContentBlocks.at(0)?.__typename !== "Note" && (
                <div
                  css={addNoteButtonStyles}
                  onClick={(): void => handleAddNote("start")}
                >
                  Add note at start
                </div>
              )}
              {blocksWithPositionInfo(localContentBlocks).map(
                ({ block, nextBlock, position, idx }) => (
                  <>
                    <div
                      key={`${block.__typename}-${idx}`}
                      css={contentBlockStyles}
                    >
                      <div css={contentMetaStyles}>
                        {Temporal.Instant.from(block.contentAt)
                          .toZonedDateTimeISO(Temporal.Now.timeZoneId())
                          .toLocaleString()}
                        {" - "}
                        {block.__typename}
                      </div>

                      {match(block)
                        .with({ __typename: "Note" }, (note) => (
                          <div css={noteContainerStyles}>
                            <textarea
                              css={inputStyles}
                              value={note.text}
                              onChange={(e): void => {
                                const updatedBlocks = [...localContentBlocks];
                                updatedBlocks[idx] = {
                                  ...note,
                                  text: e.target.value,
                                };
                                setLocalContentBlocks(updatedBlocks);
                              }}
                              rows={3}
                            />
                            <button
                              type="button"
                              onClick={(): void => handleDeleteNote(idx)}
                              css={deleteNoteButtonStyles}
                              title="Delete note"
                            >
                              ✕
                            </button>
                          </div>
                        ))
                        .with({ __typename: "Media" }, (media) => (
                          <img
                            src={media.imageSizes.fit1200.webpUrl}
                            css={mediaImageStyles}
                            alt=""
                          />
                        ))
                        .with({ __typename: "Ride" }, (ride) => (
                          <div css={rideBlockStyles}>{ride.name}</div>
                        ))
                        .exhaustive()}
                    </div>
                    {match({ block, nextBlock, position })
                      .with(
                        {
                          block: { __typename: P.not("Note") },
                          nextBlock: P.union(
                            { __typename: P.not("Note") },
                            P.nullish,
                          ),
                        },
                        () => (
                          <div
                            css={addNoteButtonStyles}
                            onClick={(): void => handleAddNote(idx)}
                          >
                            +
                          </div>
                        ),
                      )
                      .otherwise(() => (
                        <></>
                      ))}
                  </>
                ),
              )}
            </div>
          </Tabs.Content>

          <Tabs.Content value="media" css={tabContentStyles}>
            <MediaTable
              trip={tripFragment}
              onRemoveMedia={handleRemoveMedia}
              removingMedia={removingMedia}
            />
            <MediaDropzone
              tripId={trip.id}
              onUploadComplete={refetch}
              uploading={uploading}
              setUploading={setUploading}
            />
          </Tabs.Content>
        </Tabs.Root>

        <div css={buttonGroupStyles}>
          <button type="button" onClick={onClose}>
            Cancel
          </button>
          <button type="submit" disabled={loading}>
            {loading ? "Saving..." : "Save"}
          </button>
        </div>
      </form>
    </Modal>
  );
}
