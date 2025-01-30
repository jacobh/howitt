import { SerializedStyles, css } from "@emotion/react";
import { Link, useSearchParams } from "@remix-run/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { tokens } from "~/styles/tokens";
import { RouteVitals, routeSubtitleArrowCss } from "./RouteVitals";

export const RouteItemFragment = gql(`
    fragment routeItem on Route {
        id
        name
        slug
        distance
        elevationAscentM
        elevationDescentM
        isMetaComplete
    }
`);

interface Props {
  route: FragmentType<typeof RouteItemFragment>;
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
  color: ${tokens.colors.darkGrey};
`;

const subtitleContainerCss = css`
  display: flex;
`;

const routeVitalsCss = css`
  flex: 1 1 auto;
`;

export function RouteItem({
  route: routeFragment,
  titlePostfix,
  routeTitleCss,
}: Props): React.ReactNode {
  const [searchParams] = useSearchParams();
  const route = useFragment(RouteItemFragment, routeFragment);

  return (
    <div className="route-item" css={routeItemCss}>
      <p
        className="route-title"
        css={css([defaultRouteTitleCss, routeTitleCss])}
      >
        <Link to={`/routes/${route.slug}`}>{route.name}</Link>
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
