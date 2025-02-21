import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { css } from "@emotion/react";
import { useCallback, useState } from "react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { Modal } from "../../Modal";
import { MediaTable } from "./components/MediaTable";
import { MediaDropzone } from "./components/MediaDropzone";
import { RideTable } from "./components/RideTable";
import * as Tabs from "@radix-ui/react-tabs";
import { match, P } from "ts-pattern";
import { isNotNil } from "~/services/isNotNil";
import { Temporal } from "@js-temporal/polyfill";
import { ResultOf } from "@graphql-typed-document-node/core";
import { blocksWithPositionInfo } from "./utils/blocksWithPositionInfo";
import { useLocalContentBlocks } from "./hooks/useLocalContentBlocks";
import {
  tabsListStyles,
  tabsRootStyles,
  tabTriggerStyles,
} from "~/components/ui/Tabs";
import { tokens } from "~/styles/tokens";

export const EditTripFragment = gql(`
    fragment editTrip on Trip {
    id
    name 
    description
    ...tripRides
    ...tripMedia
    isPublished
    media {
      id
    }
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
  }
`);

export type EditTripFragmentValue = ResultOf<typeof EditTripFragment>;
export type TemporalContentBlockValue =
  EditTripFragmentValue["temporalContentBlocks"][number];

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

const UpdateTripMediaMutation = gql(`
  mutation UpdateTripMedia($input: UpdateTripMediaInput!) {
    updateTripMedia(input: $input) {
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
  border: 1px solid ${tokens.colors.grey300};
`;

const buttonGroupStyles = css`
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1rem;
`;

const contentTabStyles = css`
  max-height: 80vh;
  overflow: hidden;
  border: 1px solid ${tokens.colors.grey200};
`;

const contentBlockContainerStyles = css`
  display: flex;
  flex-direction: column;
  gap: 1rem;
  overflow-y: auto;
  max-height: 80vh;
  padding: 1rem;
`;

const contentBlockStyles = css`
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
`;

const contentMetaStyles = css`
  color: ${tokens.colors.grey700};
  font-size: 0.9em;
`;

const mediaImageStyles = css`
  max-width: 600px;
  border-radius: 4px;
`;

const rideBlockStyles = css`
  padding: 0.5rem;
  background-color: ${tokens.colors.grey50};
  border-radius: 4px;
`;

const addNoteButtonStyles = css`
  border: 1px dashed ${tokens.colors.grey300};
  padding: 0.5rem;
  text-align: center;
  cursor: pointer;
  color: ${tokens.colors.grey600};
  margin: 0.5rem 20px;

  &:hover {
    background-color: ${tokens.colors.grey50};
  }
`;

const noteContainerStyles = css`
  display: flex;
  gap: 0.5rem;
`;

const deleteNoteButtonStyles = css`
  background: none;
  border: none;
  color: ${tokens.colors.grey600};
  cursor: pointer;
  padding: 0.25rem;

  &:hover {
    color: #ff4444;
  }
`;

export function EditTripModal({
  trip: tripFragment,
  isOpen,
  onClose,
  refetch,
}: Props): React.ReactElement {
  const trip = useFragment(EditTripFragment, tripFragment);

  const { localContentBlocks, onCreateNote, onUpdateNote, onDeleteNote } =
    useLocalContentBlocks(trip.temporalContentBlocks);

  const [uploading, setUploading] = useState(false);
  const [name, setName] = useState(trip.name);
  const [description, setDescription] = useState(trip.description ?? "");
  const [isPublished, setIsPublished] = useState(trip.isPublished);

  const [updateTrip, { loading }] = useMutation(UpdateTripMutation, {
    onCompleted: () => {
      refetch();
      onClose();
    },
  });

  const [updateMedia, { loading: updatingMedia }] = useMutation(
    UpdateTripMediaMutation,
    {
      onCompleted: () => {
        refetch();
      },
    },
  );

  const handleRemoveMedia = useCallback(
    (mediaId: string): void => {
      const currentMediaIds = trip.media.map((m) => m.id);
      const updatedMediaIds = currentMediaIds.filter((id) => id !== mediaId);

      updateMedia({
        variables: {
          input: {
            tripId: trip.id,
            mediaIds: updatedMediaIds,
          },
        },
      });
    },
    [updateMedia, trip.id, trip.media],
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
            isPublished,
            notes,
          },
        },
      });
    },
    [trip.id, name, description, localContentBlocks, updateTrip, isPublished],
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
            <Tabs.Trigger value="rides" css={tabTriggerStyles}>
              Rides
            </Tabs.Trigger>
            <Tabs.Trigger value="media" css={tabTriggerStyles}>
              Media
            </Tabs.Trigger>
            <Tabs.Trigger value="pois" css={tabTriggerStyles}>
              POIs
            </Tabs.Trigger>
          </Tabs.List>

          <Tabs.Content value="trip">
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
            <div css={formFieldStyles}>
              <label htmlFor="isPublished">Published?</label>
              <input
                css={inputStyles}
                id="isPublished"
                type="checkbox"
                checked={isPublished}
                onChange={(e): void => setIsPublished(e.target.checked)}
              />
            </div>
            <div css={buttonGroupStyles}>
              <button type="button" onClick={onClose}>
                Cancel
              </button>
              <button type="submit" disabled={loading}>
                {loading ? "Saving..." : "Save"}
              </button>
            </div>
          </Tabs.Content>

          <Tabs.Content value="rides">
            <RideTable trip={trip} refetch={refetch} />
            <p>Toggling rides will automatically save</p>
          </Tabs.Content>

          <Tabs.Content value="content">
            <div css={contentTabStyles}>
              <div css={contentBlockContainerStyles}>
                {localContentBlocks.at(0)?.__typename !== "Note" && (
                  <button
                    css={addNoteButtonStyles}
                    onClick={(): void => onCreateNote("start")}
                  >
                    Add note at start
                  </button>
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
                                  onUpdateNote(idx, e.target.value);
                                }}
                                rows={3}
                              />
                              <button
                                type="button"
                                onClick={(): void => onDeleteNote(idx)}
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
                            <button
                              css={addNoteButtonStyles}
                              onClick={(): void => onCreateNote(idx)}
                            >
                              +
                            </button>
                          ),
                        )
                        .otherwise(() => (
                          <></>
                        ))}
                    </>
                  ),
                )}
              </div>
            </div>
            <div css={buttonGroupStyles}>
              <button type="button" onClick={onClose}>
                Cancel
              </button>
              <button type="submit" disabled={loading}>
                {loading ? "Saving..." : "Save"}
              </button>
            </div>
          </Tabs.Content>

          <Tabs.Content value="media">
            <MediaTable
              trip={trip}
              onRemoveMedia={handleRemoveMedia}
              removingMedia={updatingMedia}
            />
            <MediaDropzone
              tripId={trip.id}
              onUploadComplete={refetch}
              uploading={uploading}
              setUploading={setUploading}
            />
          </Tabs.Content>
          <Tabs.Content value="pois">
            <p>Coming soon</p>
          </Tabs.Content>
        </Tabs.Root>
      </form>
    </Modal>
  );
}
