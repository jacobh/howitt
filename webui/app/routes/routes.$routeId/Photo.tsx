import { css } from "@emotion/react";
import { Photo as ApiPhoto } from "~/__generated__/graphql";

interface Props {
  photo: Pick<ApiPhoto, "url" | "caption">;
}

const photoContainerCss = css`
  margin: 20px 0;
`;

const photoCss = css`
  max-width: 100%;
  max-height: 70vh;

  margin: 0 auto;
`;

const captionCss = css`
  text-align: center;
`;

export function Photo({ photo: { url, caption } }: Props): React.ReactElement {
  return (
    <div css={photoContainerCss}>
      <img css={photoCss} src={url} alt={caption ?? undefined}></img>
      {caption ? <p css={captionCss}>{caption}</p> : <></>}
    </div>
  );
}
