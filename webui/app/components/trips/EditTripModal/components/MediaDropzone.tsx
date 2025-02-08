import { css } from "@emotion/react";
import { useCallback } from "react";
import { useDropzone } from "react-dropzone-esm";
import Cookies from "js-cookie";
import { getApiBaseUrl } from "~/env.client";

const dropzoneStyles = css({
  border: "2px dashed #cccccc",
  borderRadius: "4px",
  padding: "20px",
  textAlign: "center",
  background: "#ffffff",
  cursor: "pointer",
  marginTop: "16px",
  "&:hover": {
    borderColor: "#999999",
  },
});

const dragActiveStyles = css({
  background: "#f0f0f0",
});

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

  return (
    <div
      {...getRootProps()}
      css={[dropzoneStyles, isDragActive && dragActiveStyles]}
    >
      <input {...getInputProps()} />
      {uploading ? (
        <p>Uploading files...</p>
      ) : isDragActive ? (
        <p>Drop the files here ...</p>
      ) : (
        <p>Drag &amp; drop some files here, or click to select files</p>
      )}
    </div>
  );
}
