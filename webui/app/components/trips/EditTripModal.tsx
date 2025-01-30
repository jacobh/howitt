import { useMutation } from "@apollo/client";
import { css } from "@emotion/react";
import Cookies from "js-cookie";
import { useCallback, useState } from "react";
import { useDropzone } from "react-dropzone";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { getApiBaseUrl } from "~/env.client";
import { Modal } from "../Modal";

export const EditTripFragment = gql(`
    fragment editTrip on Trip {
    id
    name 
    description
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

  const onDrop = useCallback(
    async (acceptedFiles: File[]) => {
      setUploading(true);

      try {
        for (const file of acceptedFiles) {
          const formData = new FormData();
          formData.append("file", file);
          formData.append("name", file.name);
          formData.append("relation_ids", JSON.stringify([trip.id]));

          const response = await fetch(`${getApiBaseUrl()}/upload/media`, {
            method: "POST",
            body: formData,
            headers: {
              Authorization: `Bearer ${Cookies.get("token")}`,
            },
          });

          if (!response.ok) throw new Error("Upload failed");
        }
        refetch();
      } catch (error) {
        console.error("Upload failed:", error);
        // You might want to show an error message to the user here
      } finally {
        setUploading(false);
      }
    },
    [trip.id, refetch],
  );

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: {
      "image/*": [".jpeg", ".jpg", ".png", ".gif"],
    },
  });

  const dropzoneStyles = css`
    border: 2px dashed #cccccc;
    border-radius: 4px;
    padding: 20px;
    text-align: center;
    background: ${isDragActive ? "#f0f0f0" : "#ffffff"};
    cursor: pointer;
    margin-top: 16px;

    &:hover {
      border-color: #999999;
    }
  `;

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

  const [removeMedia, { loading: removingMedia }] = useMutation(
    RemoveTripMediaMutation,
    {
      onCompleted: () => {
        refetch();
      },
    },
  );

  const handleDeleteMedia = (mediaId: string): void => {
    removeMedia({
      variables: {
        input: {
          tripId: trip.id,
          mediaIds: [mediaId],
        },
      },
    });
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <form onSubmit={handleSubmit} css={formStyles}>
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
          <label htmlFor="media">Media</label>
          <div>
            <table
              css={css`
                width: 100%;
                border-collapse: collapse;

                th,
                td {
                  padding: 8px;
                  text-align: left;
                  border-bottom: 1px solid #ddd;
                }

                th {
                  background-color: #f5f5f5;
                  font-weight: 500;
                }
              `}
            >
              <thead>
                <tr>
                  <th>Thumbnail</th>
                  <th>Path</th>
                  <th>Created At</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {trip.media.map((media) => (
                  <tr key={media.id}>
                    <td css={{ width: "120px" }}>
                      <img
                        src={media.imageSizes.fill600.webpUrl}
                        css={{
                          width: "100px",
                          height: "100px",
                          objectFit: "cover",
                          borderRadius: "4px",
                        }}
                        alt=""
                      />
                    </td>
                    <td>{media.path}</td>
                    <td>
                      {new Date(media.createdAt).toLocaleDateString("en-US")}
                    </td>
                    <td>
                      <button
                        type="button"
                        onClick={(): void => handleDeleteMedia(media.id)}
                        disabled={removingMedia}
                        css={css`
                          padding: 4px 8px;
                          background-color: transparent;
                          color: #666;
                          border: 1px solid #ccc;
                          border-radius: 4px;
                          cursor: pointer;
                          font-size: 0.875rem;

                          &:hover {
                            background-color: #f5f5f5;
                            color: #ff4444;
                            border-color: #ff4444;
                          }

                          &:disabled {
                            background-color: #f5f5f5;
                            color: #999;
                            border-color: #ddd;
                            cursor: not-allowed;
                          }
                        `}
                      >
                        Delete
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
            <div {...getRootProps()} css={dropzoneStyles}>
              <input {...getInputProps()} />
              {uploading ? (
                <p>Uploading files...</p>
              ) : isDragActive ? (
                <p>Drop the files here ...</p>
              ) : (
                <p>Drag 'n' drop some files here, or click to select files</p>
              )}
            </div>
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
      </form>
    </Modal>
  );
}
