import { css } from "@emotion/react";
import { useEffect, useState } from "react";

import { LoadingSpinner } from "~/components/ui/LoadingSpinner";
import { tokens } from "~/styles/tokens";

function useCheckImage(url: string): {
  isReady: boolean;
  url: string | undefined;
} {
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    if (!url || isReady) return;

    let timeoutId: NodeJS.Timeout;

    const checkImage = (): void => {
      const img = new Image();
      img.onload = (): void => {
        setIsReady(true);
      };
      img.src = url;

      if (!isReady) {
        timeoutId = setTimeout(checkImage, 1000);
      }
    };

    checkImage(); // Initial check

    return (): void => clearTimeout(timeoutId);
  }, [url, isReady]);

  return { isReady, url: isReady ? url : undefined };
}

const thumbnailImageCss = css({
  width: "100px",
  height: "100px",
  objectFit: "cover",
  borderRadius: "4px",
});

const loadingCss = css(thumbnailImageCss, {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  background: tokens.colors.grey50,
});

interface ThumbnailImageProps {
  url: string;
}

export function ThumbnailImage({
  url,
}: ThumbnailImageProps): React.ReactElement {
  const { isReady, url: checkedUrl } = useCheckImage(url);

  if (isReady) {
    return <img src={checkedUrl} css={thumbnailImageCss} alt="" />;
  }

  return (
    <div css={loadingCss}>
      <LoadingSpinner />
    </div>
  );
}
