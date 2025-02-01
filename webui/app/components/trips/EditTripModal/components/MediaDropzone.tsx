import { css } from "@emotion/react";
import { useCallback } from "react";
import { useDropzone } from "react-dropzone";
import Cookies from "js-cookie";
import { getApiBaseUrl } from "~/env.client";

interface Props {
  tripId: string;
  onUploadComplete: () => void;
  uploading: boolean;
  setUploading: (uploading: boolean) => void;
}

export class MediaUploadClient {
  constructor(
    private readonly apiBaseUrl: string,
    private readonly token: string,
  ) {}

  async uploadMedia(file: File, relationIds: string[]): Promise<void> {
    const formData = new FormData();
    formData.append("file", file);
    formData.append("name", file.name);
    formData.append("relation_ids", JSON.stringify(relationIds));

    const response = await fetch(`${this.apiBaseUrl}/upload/media`, {
      method: "POST",
      body: formData,
      headers: {
        Authorization: `Bearer ${this.token}`,
      },
    });

    if (!response.ok) throw new Error("Upload failed");
  }
}

export function MediaDropzone({
  tripId,
  onUploadComplete,
  uploading,
  setUploading,
}: Props): React.ReactElement {
  const onDrop = useCallback(
    async (acceptedFiles: File[]) => {
      setUploading(true);

      try {
        const client = new MediaUploadClient(
          getApiBaseUrl(),
          Cookies.get("token") ?? "",
        );

        for (const file of acceptedFiles) {
          await client.uploadMedia(file, [tripId]);
        }

        onUploadComplete();
      } catch (error) {
        console.error("Upload failed:", error);
      } finally {
        setUploading(false);
      }
    },
    [tripId, onUploadComplete, setUploading],
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

  return (
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
  );
}
