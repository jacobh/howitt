import { Photo } from "~/__generated__/graphql";

interface ResizeOptions {
  width: number;
  height: number;
}

export enum PhotoSize {
  Large,
}

const SIZE_DIMENSIONS_MAP: Record<PhotoSize, ResizeOptions> = {
  [PhotoSize.Large]: { width: 1600, height: 1600 },
};

const BASE_URL = "https://d330luy891602k.cloudfront.net";

export function generatePhotoUrl(
  photo: Pick<Photo, "id">,
  size: PhotoSize
): string {
  const ulid = photo.id.split("#")[1];

  return [
    BASE_URL,
    btoa(
      JSON.stringify({
        bucket: "howitt-photos",
        key: `source/${ulid}.jpg`,
        edits: {
          resize: { fit: "outside", ...SIZE_DIMENSIONS_MAP[size] },
          webp: {
            preset: "photo",
            effort: 4,
          },
        },
      })
    ),
  ].join("/");
}
