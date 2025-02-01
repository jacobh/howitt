import { useMutation } from "@apollo/client";
import { css } from "@emotion/react";
import { useState } from "react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { Modal } from "../../Modal";
import { MediaTable } from "./components/MediaTable";
import { MediaDropzone } from "./components/MediaDropzone";
import { TabItem } from "./components/TabItem";
import { TabList } from "./components/TabList";

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

export function EditTripModal({
  trip: tripFragment,
  isOpen,
  onClose,
  refetch,
}: Props): React.ReactElement {
  const trip = useFragment(EditTripFragment, tripFragment);
  const [uploading, setUploading] = useState(false);
  const [name, setName] = useState(trip.name);
  const [description, setDescription] = useState(trip.description ?? "");

  const [updateTrip, { loading }] = useMutation(UpdateTripMutation, {
    onCompleted: () => {
      onClose();
    },
  });

  const [removeMedia, { loading: removingMedia }] = useMutation(
    RemoveTripMediaMutation,
    {
      onCompleted: () => {
        refetch();
      },
    },
  );

  const handleRemoveMedia = (mediaId: string): void => {
    removeMedia({
      variables: {
        input: {
          tripId: trip.id,
          mediaIds: [mediaId],
        },
      },
    });
  };

  const handleSubmit = (e: React.FormEvent): void => {
    e.preventDefault();

    updateTrip({
      variables: {
        input: {
          tripId: trip.id,
          name,
          description: description || null,
        },
      },
    });
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <form onSubmit={handleSubmit} css={formStyles}>
        <TabList>
          <TabItem label="Trip">
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
          </TabItem>

          <TabItem label="Content">
            <h2>Content</h2>
            <div
              css={{ display: "flex", flexDirection: "column", gap: "1rem" }}
            >
              {trip.temporalContentBlocks.map((block, index) => (
                <div
                  key={`${block.__typename}-${index}`}
                  css={{
                    display: "flex",
                    flexDirection: "column",
                    gap: "0.5rem",
                  }}
                >
                  <div css={{ color: "#666", fontSize: "0.9em" }}>
                    {new Date(block.contentAt).toLocaleString()}
                    {" - "}
                    {block.__typename}
                  </div>

                  {block.__typename === "Note" && (
                    <textarea
                      css={inputStyles}
                      value={block.text}
                      onChange={(e): void => {
                        // TODO: Implement note updating
                        console.log("Update note:", e.target.value);
                      }}
                      rows={3}
                    />
                  )}

                  {block.__typename === "Media" && (
                    <img
                      src={block.imageSizes.fit1200.webpUrl}
                      css={{ maxWidth: "600px", borderRadius: "4px" }}
                      alt=""
                    />
                  )}

                  {block.__typename === "Ride" && (
                    <div
                      css={{
                        padding: "0.5rem",
                        backgroundColor: "#f5f5f5",
                        borderRadius: "4px",
                      }}
                    >
                      {block.name}
                    </div>
                  )}
                </div>
              ))}
            </div>
          </TabItem>

          <TabItem label="Media">
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
          </TabItem>
        </TabList>

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
