import { css } from "@emotion/react";
import { Route } from "~/__generated__/graphql";
import { formatDistance, formatVertical } from "~/services/format";
import { COLORS } from "~/styles/theme";

interface Props {
  route: Pick<Route, "distance" | "elevationAscentM" | "elevationDescentM">;
}

const routeSubtitleCss = css`
  color: ${COLORS.midGrey};

  display: grid;
  grid-auto-flow: column;
  max-width: 320px;

  font-size: 0.875rem; /* 14px */
  line-height: 1.25rem; /* 20px */
`;
const routeSubtitleArrowCss = css`
  width: 30px;
  display: inline-block;
  text-align: center;
`;

export function RouteVitals({ route }: Props): JSX.Element {
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
