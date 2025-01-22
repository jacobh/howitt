import { css } from "@emotion/react";
import { Link } from "@remix-run/react";

import { PropsWithChildren } from "react";
import { makeMqs } from "~/styles/mediaQueries";

export { Nav } from "./Nav";

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

export function Container({ children }: PropsWithChildren): React.ReactNode {
  return <div css={containerCss}>{children}</div>;
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

const titleSegmentCss = css`
  flex-shrink: 1;
`;

const titleSeparatorCss = css`
  margin: 0 4px 0 8px;
`;

interface Props {
  titleSegments: TitleSegment[];
}

interface TitleSegment {
  name: string;
  linkTo: string;
}

export function SidebarContainer({
  titleSegments,
  children,
}: PropsWithChildren<Props>): React.ReactNode {
  return (
    <div css={sidebarContainerOuterCss}>
      <div css={sidebarContainerInnerCss}>
        <h3 css={sidebarTitleCss}>
          {titleSegments.map((segment, index) => (
            <>
              {index > 0 && <span css={titleSeparatorCss}>/</span>}

              <Link to={segment.linkTo} css={[titleSegmentCss]}>
                {segment.name}
              </Link>
            </>
          ))}
        </h3>
        <hr />
        <div css={sidebarChildrenCss}>{children}</div>
      </div>
    </div>
  );
}

export function MapContainer({ children }: PropsWithChildren): React.ReactNode {
  return <div css={mapContainerCss}>{children}</div>;
}
