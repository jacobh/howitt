import { css } from "@emotion/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { buttonStyles } from "~/components/ui/Button";

export const TripMediaFragment = gql(`
  fragment tripMedia on Trip {
    id
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
interface Props {
  trip: FragmentType<typeof TripMediaFragment>;
  onRemoveMedia: (mediaId: string) => void;
  removingMedia: boolean;
}

const mediaTableContainerCss = css`
  max-height: 67vh;
  overflow: hidden;
  border: 1px solid #ddd;
`;

const mediaTableCss = css`
  width: 100%;
  border-collapse: separate;
  border-spacing: 0;

  th,
  td {
    padding: 8px;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }

  th {
    background-color: #f5f5f5;
    font-weight: 500;
    position: sticky;
    top: 0;
    z-index: 1;
  }

  th:nth-of-type(1),
  td:nth-of-type(1) {
    width: 120px;
  } /* Thumbnail */
  th:nth-of-type(2),
  td:nth-of-type(2) {
    width: auto;
  } /* Path - takes remaining space */
  th:nth-of-type(3),
  td:nth-of-type(3) {
    width: 100px;
  } /* Created At */
  th:nth-of-type(4),
  td:nth-of-type(4) {
    width: 80px;
  } /* Actions */

  td:nth-of-type(2) {
    white-space: normal;
    word-break: break-all;
  }

  tbody {
    display: block;
    overflow-y: auto;
    max-height: calc(67vh - 41px); /* 41px accounts for header height */
  }

  thead,
  tbody tr {
    display: table;
    width: 100%;
    table-layout: fixed;
  }
`;

const deleteButtonCss = css(
  buttonStyles,
  css`
    padding: 4px 8px;
    color: #666;
    font-size: 0.875rem;

    &:hover {
      color: #ff4444;
      border-color: #ff4444;
    }

    &:disabled {
      background-color: #f5f5f5;
      color: #999;
      border-color: #ddd;
    }
  `,
);

const thumbnailCellCss = css({
  width: "120px",
});

const thumbnailImageCss = css({
  width: "100px",
  height: "100px",
  objectFit: "cover",
  borderRadius: "4px",
});

const getFileName = (path: string): string => {
  return path.split("/").pop() || path;
};

export function MediaTable({
  trip: tripFragment,
  onRemoveMedia,
  removingMedia,
}: Props): React.ReactElement {
  const trip = useFragment(TripMediaFragment, tripFragment);

  return (
    <div css={mediaTableContainerCss}>
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
              <td css={thumbnailCellCss}>
                <img
                  src={media.imageSizes.fill600.webpUrl}
                  css={thumbnailImageCss}
                  alt=""
                />
              </td>
              <td>{getFileName(media.path)}</td>
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
    </div>
  );
}
