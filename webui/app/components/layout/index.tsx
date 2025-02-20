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
    overflow-y: auto;
  `,
]);

const mapContainerCss = makeMqs([
  css`
    grid-area: map;
    width: 100vw;
    height: 100%;

    &.overlay {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      height: 70vh;
      z-index: 2;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }
  `,
  css``,
  css``,
  css``,
  css`
    width: 100%;
    &.overlay {
      position: static;
      height: 100%;
      box-shadow: none;
    }
  `,
]);

const mapOverlayMaskCss = makeMqs([
  css`
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: black;
    opacity: 0.2;
    z-index: 1;
    cursor: pointer;
  `,
  css``,
  css``,
  css``,
  css`
    display: none;
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
    min-height: min-content;
  `,
]);

const sidebarTitleCss = css`
  margin-bottom: 12px;
  display: flex;
`;

const sidebarChildrenCss = makeMqs([css``, css``, css``, css``, css``]);
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

interface MapContainerProps extends PropsWithChildren {
  isOverlayActive?: boolean;
  onDismissOverlay?: () => void;
}

export function MapContainer({
  children,
  onDismissOverlay,
  isOverlayActive = false,
}: MapContainerProps): React.ReactNode {
  return (
    <>
      {isOverlayActive && (
        // eslint-disable-next-line jsx-a11y/click-events-have-key-events, jsx-a11y/no-static-element-interactions
        <div css={mapOverlayMaskCss} onClick={onDismissOverlay} />
      )}
      <div css={mapContainerCss} className={isOverlayActive ? "overlay" : ""}>
        {children}
      </div>
    </>
  );
}
