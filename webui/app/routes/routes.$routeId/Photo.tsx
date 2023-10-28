import { css } from "@emotion/react";
import { Photo as ApiPhoto } from "~/__generated__/graphql";
import { PhotoSize, generatePhotoUrl } from "~/services/photos";

interface Props {
  photo: Pick<ApiPhoto, "id" | "caption">;
}

const photoCss = css`
  max-width: 100%;
  max-height: 70vh;

  margin: 0 auto;
`;

const captionCss = css`
  text-align: center;
`;

export function Photo({ photo: { id, caption } }: Props): React.ReactElement {
  return (
    <div>
      <img
        css={photoCss}
        src={generatePhotoUrl({ id }, PhotoSize.Large)}
        alt={caption ?? undefined}
      ></img>
      {caption ? <p css={captionCss}>{caption}</p> : <></>}
    </div>
  );
}
