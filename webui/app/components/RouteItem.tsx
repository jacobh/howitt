import { css } from "@emotion/react";
import { Link } from "@remix-run/react";
import { Route } from "~/__generated__/graphql";
import { COLORS } from "~/styles/theme";
import { RouteVitals } from "./RouteVitals";

interface Props {
  route: Pick<
    Route,
    "id" | "name" | "distance" | "elevationAscentM" | "elevationDescentM"
  >;
}

const routeItemCss = css`
  padding: 20px 0;
  container-type: inline-size;
  border-bottom: 1px solid ${COLORS.offWhite};
`;

const routeTitleCss = css({ marginBottom: "6px", textDecoration: "underline" });

export function RouteItem({ route }: Props): JSX.Element {
  return (
    <div className="route-item" css={routeItemCss}>
      <h3 className="route-title" css={routeTitleCss}>
        <Link to={`/routes/${route.id.split("#")[1]}`}>{route.name}</Link>
      </h3>
      <RouteVitals route={route} />
    </div>
  );
}
