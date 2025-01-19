import { SerializedStyles, css } from "@emotion/react";
import { Link, useSearchParams } from "@remix-run/react";
import { Route } from "~/__generated__/graphql";
import { COLORS } from "~/styles/theme";
import { RouteVitals, routeSubtitleArrowCss } from "./RouteVitals";

interface Props {
  route: Pick<
    Route,
    "id" | "name" | "distance" | "elevationAscentM" | "elevationDescentM"
  > &
    Partial<Route>;
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

const subtitleContainerCss = css`
  display: flex;
`;

const routeVitalsCss = css`
  flex: 1 1 auto;
`;

export function RouteItem({
  route,
  titlePostfix,
  routeTitleCss,
}: Props): JSX.Element {
  const [searchParams] = useSearchParams();

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
      <div css={subtitleContainerCss}>
        <div css={routeVitalsCss}>
          <RouteVitals route={route} />
        </div>
        {searchParams.has("debug") && route.isMetaComplete ? (
          <span>
            <span css={routeSubtitleArrowCss}>&#x2713;</span>
          </span>
        ) : null}
      </div>
    </div>
  );
}
