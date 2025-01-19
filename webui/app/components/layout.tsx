import { css } from "@emotion/react";
import { Link } from "@remix-run/react";

import { PropsWithChildren } from "react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { makeMqs } from "~/styles/mediaQueries";

const containerCss = makeMqs([
  css`
    display: grid;
    grid-auto-flow: row;

    width: 100vw;
    height: 100vh;

    grid-template-areas:
      "nav"
      "map"
      "sidebar";

    grid-template-rows: 50px 66vh minmax(calc(34vh - 50px), auto);
  `,
  css``,
  css``,
  css``,
  css`
    grid-template-rows: 50px min-content;
    grid-template-columns: minmax(640px, 40%) 1fr;
    grid-template-areas:
      "nav nav"
      "sidebar map";
  `,
  css``,
]);

export function Container({ children }: PropsWithChildren): JSX.Element {
  return <div css={containerCss}>{children}</div>;
}

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
`;

const userInfoCss = css`
  margin-left: auto;
  height: 100%;
`;

export function Nav(props: NavProps): JSX.Element {
  const viewer = useFragment(ViewerInfoFragment, props.viewer);

  return (
    <nav css={navCss}>
      <h2 css={logoCss}>Howitt Plains</h2>
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

const sidebarContainerOuterCss = makeMqs([
  css`
    grid-area: sidebar;
    width: 100vw;
    box-shadow: 0px -1px 5px #9d9d9d;
    z-index: 1;
  `,
  css``,
  css``,
  css``,
  css`
    width: 100%;
    height: calc(100vh - 50px);
    box-shadow: none;
  `,
]);

const mapContainerCss = makeMqs([
  css`
    grid-area: map;
    width: 100vw;
    height: 100%;
  `,
  css``,
  css``,
  css``,
  css`
    width: 100%;
  `,
]);

const sidebarContainerInnerCss = makeMqs([
  css`
    padding: 12px 4%;
  `,
  css`
    padding: 12px 6%;
  `,
  css`
    padding: 14px 9%;
  `,
  css`
    padding: 16px 12%;
  `,
  css`
    padding: max(18px, 1vw) 6%;
    height: inherit;
    display: flex;
    flex-direction: column;
  `,
]);

const sidebarTitleCss = css`
  margin-bottom: 12px;
  display: flex;
`;

const sidebarChildrenCss = makeMqs([
  css``,
  css``,
  css``,
  css``,
  css`
    overflow-x: hidden;
    overflow-y: scroll;
  `,
]);

const titlePostfixCss = css`
  margin-left: 8px;
`;

interface Props {
  title: string;
  titleLinkTo?: string;
  titlePostfix?: string;
  username?: string;
}

export function SidebarContainer({
  title,
  titlePostfix,
  titleLinkTo,
  children,
}: PropsWithChildren<Props>): JSX.Element {
  return (
    <div css={sidebarContainerOuterCss}>
      <div css={sidebarContainerInnerCss}>
        <h3 css={sidebarTitleCss}>
          {titleLinkTo ? (
            <Link to={titleLinkTo} css={{ flexShrink: 1 }}>
              {title}
            </Link>
          ) : (
            <span css={{ flexShrink: 1 }}>{title}</span>
          )}
          {titlePostfix ? (
            <span css={titlePostfixCss}>{titlePostfix}</span>
          ) : (
            <></>
          )}
        </h3>
        <hr />
        <div css={sidebarChildrenCss}>{children}</div>
      </div>
    </div>
  );
}

export function MapContainer({ children }: PropsWithChildren): JSX.Element {
  return <div css={mapContainerCss}>{children}</div>;
}
