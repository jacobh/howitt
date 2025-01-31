import { css } from "@emotion/react";
import { FragmentType, useFragment } from "~/__generated__";
import { EditTripFragment } from "..";

interface Props {
  trip: FragmentType<typeof EditTripFragment>;
  onRemoveMedia: (mediaId: string) => void;
  removingMedia: boolean;
}

const mediaTableCss = css`
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
`;

const deleteButtonCss = css`
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
`;

export function MediaTable({
  trip: tripFragment,
  onRemoveMedia,
  removingMedia,
}: Props): React.ReactElement {
  const trip = useFragment(EditTripFragment, tripFragment);

  return (
    <table css={mediaTableCss}>
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
            <td>{new Date(media.createdAt).toLocaleDateString("en-US")}</td>
            <td>
              <button
                type="button"
                onClick={(): void => onRemoveMedia(media.id)}
                disabled={removingMedia}
                css={deleteButtonCss}
              >
                Delete
              </button>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
