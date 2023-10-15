import { SerializedStyles, css } from "@emotion/react";
import { Link } from "@remix-run/react";
import { Route } from "~/__generated__/graphql";
import { COLORS } from "~/styles/theme";
import { RouteVitals } from "./RouteVitals";

interface Props {
  route: Pick<
    Route,
    "id" | "name" | "distance" | "elevationAscentM" | "elevationDescentM"
  >;
  routeTitleCss?: SerializedStyles;
  titlePostfix?: string;
}

const routeItemCss = css`
  container-type: inline-size;
`;

const defaultRouteTitleCss = css({
  marginBottom: "6px",
});

const titlePostfixCss = css`
  text-decoration: none;
  color: ${COLORS.darkGrey};
`;

export function RouteItem({
  route,
  titlePostfix,
  routeTitleCss,
}: Props): JSX.Element {
  return (
    <div className="route-item" css={routeItemCss}>
      <p
        className="route-title"
        css={css([defaultRouteTitleCss, routeTitleCss])}
      >
        <Link to={`/routes/${route.id.split("#")[1]}`}>{route.name}</Link>
        {titlePostfix && (
          <span css={titlePostfixCss}>&nbsp;&nbsp;{titlePostfix}</span>
        )}
      </p>
      <RouteVitals route={route} />
    </div>
  );
}
