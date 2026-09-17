import { css } from "@emotion/react";
import { Link } from "react-router";

import { makeMqs } from "~/styles/mediaQueries";
import { useViewer } from "./Viewer";

const navCss = makeMqs([
  css`
    grid-area: nav;
    height: 50px;
    display: flex;
    line-height: 50px;
    box-shadow: 0px -1px 5px #9d9d9d;
    z-index: 1;
    padding: 0px 1.5vw;
  `,
]);

const logoCss = makeMqs([
  css`
    font-size: 20px !important;
    line-height: 50px !important;

    margin-right: 2vw;

    a {
      text-decoration: none;

      &:hover {
        text-decoration: underline;
      }
    }
  `,
  css``,
  css``,
  css``,
  css`
    margin-right: 1.8vw;
  `,
  css`
    margin-right: 1.5vw;
  `,
]);

const navLinkCss = makeMqs([
  css`
    font-size: 18px !important;
    line-height: 50px !important;

    margin-right: 1.5vw;
  `,
  css``,
  css``,
  css`
    margin-right: 0.9vw;
  `,
  css`
    margin-right: 0.8vw;
  `,
]);

const userInfoCss = css`
  margin-left: auto;
  height: 100%;
`;

export function Nav(): React.ReactNode {
  const viewer = useViewer();

  return (
    <nav css={navCss}>
      <h2 css={logoCss}>
        <Link to="/">Howitt Plains</Link>
      </h2>
      <h3 css={navLinkCss}>
        <Link to="/trips">Trips</Link>
      </h3>
      <h3 css={navLinkCss}>
        <Link to="/routes">Routes</Link>
      </h3>
      <div css={userInfoCss}>
        {viewer.status === "authenticated" ? (
          <Link to="/workshop">{viewer.viewer.profile.username}</Link>
        ) : viewer.status === "anonymous" ? (
          <Link to="/login">Login</Link>
        ) : undefined}
      </div>
    </nav>
  );
}
