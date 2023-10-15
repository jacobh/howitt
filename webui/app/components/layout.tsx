import { css } from "@emotion/react";
import { Link } from "@remix-run/react";

import { PropsWithChildren } from "react";
import { makeMqs } from "~/styles/mediaQueries";

const containerCss = makeMqs([
  css`
    display: grid;
    grid-auto-flow: row;

    width: 100vw;
    height: 100vh;
  `,
  css``,
  css``,
  css``,
  css`
    grid-auto-flow: column;
    grid-template-columns: minmax(640px, 40%) 1fr;
  `,
  css``,
]);

export function Container({ children }: PropsWithChildren): JSX.Element {
  return <div css={containerCss}>{children}</div>;
}

const sidebarContainerOuterCss = makeMqs([
  css`
    grid-row: 2;
    width: 100vw;
  `,
  css``,
  css``,
  css``,
  css`
    grid-row: unset;
    width: 100%;
    height: 100vh;
  `,
]);

const mapContainerCss = makeMqs([
  css`
    grid-row: 1;
    width: 100vw;
    height: 66vh;
  `,
  css``,
  css``,
  css``,
  css`
    grid-row: unset;
    width: 100%;
    height: 100vh;
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
    height: 100vh;
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
    height: auto;
    overflow-x: hidden;
    overflow-y: scroll;
  `,
]);

export function SidebarContainer({
  title,
  showBack,
  children,
}: PropsWithChildren<{ title: string; showBack?: boolean }>): JSX.Element {
  return (
    <div css={sidebarContainerOuterCss}>
      <div css={sidebarContainerInnerCss}>
        <h3 css={sidebarTitleCss}>
          {showBack && (
            <span css={{ display: "inline-flex" }}>
              <Link to="/">Routes</Link>
              <span css={{ padding: "0 6px" }}>/</span>
            </span>
          )}
          <span css={{ flexShrink: 1 }}>{title}</span>
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
