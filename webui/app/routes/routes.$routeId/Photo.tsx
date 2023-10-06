import { Photo as ApiPhoto } from "~/__generated__/graphql";

interface Props {
  photo: Pick<ApiPhoto, "url" | "caption">;
}

export function Photo({ photo: { url, caption } }: Props): React.ReactElement {
  return (
    <div>
      <img src={url} alt={caption ?? undefined}></img>
      {caption ? <p>{caption}</p> : <></>}
    </div>
  );
}
