import { css } from "@emotion/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { buttonStyles } from "~/components/ui/Button";
import { ThumbnailImage } from "./ThumbnailImage";
import { useMemo } from "react";
import { tableContainerCss, tableCss } from "~/components/ui/Table";
import { tokens } from "~/styles/tokens";

export const TripMediaFragment = gql(`
  fragment tripMedia on Trip {
    id
    media {
      id
      path
      createdAt
      capturedAt
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

const mediaTableCss = css(
  tableCss,
  css`
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
  `,
);

const deleteButtonCss = css(
  buttonStyles,
  css`
    padding: 4px 8px;
    color: ${tokens.colors.grey500};
    font-size: 0.875rem;

    &:hover {
      color: #ff4444;
      border-color: #ff4444;
    }

    &:disabled {
      background-color: ${tokens.colors.grey50};
      color: ${tokens.colors.grey400};
      border-color: ${tokens.colors.grey200};
    }
  `,
);

const thumbnailCellCss = css({
  width: "120px",
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

  const sortedMedia = useMemo(() => {
    return [...trip.media].sort((a, b) => {
      return (
        new Date(b.capturedAt).getTime() - new Date(a.capturedAt).getTime()
      );
    });
  }, [trip.media]);

  return (
    <div css={tableContainerCss}>
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
          {sortedMedia.map((media) => (
            <tr key={media.id}>
              <td css={thumbnailCellCss}>
                <ThumbnailImage url={media.imageSizes.fill600.webpUrl} />
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
