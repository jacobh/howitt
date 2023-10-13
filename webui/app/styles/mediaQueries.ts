import { SerializedStyles, css } from "@emotion/react";
import { isNil, zip } from "lodash";

const breakpoints = [0, 640, 768, 1024, 1280, 1536, 1920];

export function makeMqs(styles: SerializedStyles[]): SerializedStyles {
  return css(
    zip(breakpoints, styles).map(([bp, style]) =>
      !isNil(bp) && !isNil(style)
        ? css`
            @media (min-width: ${bp}px) {
              ${style}
            }
          `
        : undefined
    )
  );
}
