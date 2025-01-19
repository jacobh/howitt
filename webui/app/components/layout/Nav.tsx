import { css } from "@emotion/react";
import { Link } from "@remix-run/react";

import { FragmentType, gql, useFragment } from "~/__generated__";
import { makeMqs } from "~/styles/mediaQueries";

export const ViewerInfoFragment = gql(`
    fragment viewerInfo on Viewer {
        id
        profile {
          username
        }
    }
  `);

type NavProps = {
  viewer?: FragmentType<typeof ViewerInfoFragment> | null;
};

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

const logoCss = css`
  font-size: 20px !important;
  line-height: 50px !important;

  a {
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  }
`;

const userInfoCss = css`
  margin-left: auto;
  height: 100%;
`;

export function Nav(props: NavProps): JSX.Element {
  const viewer = useFragment(ViewerInfoFragment, props.viewer);

  return (
    <nav css={navCss}>
      <h2 css={logoCss}>
        <Link to="/">Howitt Plains</Link>
      </h2>
      <div css={userInfoCss}>
        {viewer ? (
          <Link to={`/riders/${viewer.profile.username}`}>
            {viewer.profile.username}
          </Link>
        ) : (
          <Link to="/login">Login</Link>
        )}
      </div>
    </nav>
  );
}
