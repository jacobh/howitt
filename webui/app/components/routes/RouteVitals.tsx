import { css } from "@emotion/react";
import { Route } from "~/__generated__/graphql";
import { formatDistance, formatVertical } from "~/services/format";
import { tokens } from "~/styles/tokens";

interface Props {
  route: Pick<Route, "distance" | "elevationAscentM" | "elevationDescentM">;
}

const routeSubtitleCss = css`
  color: ${tokens.colors.midGrey};

  display: grid;
  grid-auto-flow: column;
  max-width: 320px;

  font-size: 0.875rem; /* 14px */
  line-height: 1.25rem; /* 20px */
`;
export const routeSubtitleArrowCss = css`
  color: ${tokens.colors.midGrey};

  width: 30px;
  display: inline-block;
  text-align: center;
`;

export function RouteVitals({ route }: Props): React.ReactNode {
  return (
    <p css={routeSubtitleCss}>
      <span>
        <span css={routeSubtitleArrowCss}>&rarr;</span>
        {formatDistance(route.distance)}
      </span>
      <span>
        <span css={routeSubtitleArrowCss}>&uarr;</span>
        {formatVertical(route.elevationAscentM)}
      </span>
      <span>
        <span css={routeSubtitleArrowCss}>&darr;</span>
        {formatVertical(route.elevationDescentM)}
      </span>
    </p>
  );
}
