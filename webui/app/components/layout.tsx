import { css } from "@emotion/react";
import { Link } from "@remix-run/react";
import { PropsWithChildren } from "react";

export function Container({ children }: PropsWithChildren): JSX.Element {
  return <div className="grid grid-cols-12">{children}</div>;
}

const sidebarContainerOuterCss = css`
  height: 100vh;
  overflow-y: scroll;

  grid-column: span 4 / span 4;

  @media (min-width: 2100px) {
    grid-column: span 3 / span 3;
  }
`;

const mapContainerCss = css`
  grid-column: span 8 / span 8;

  @media (min-width: 2100px) {
    grid-column: span 9 / span 9;
  }
`;

const sidebarContainerInnerCss = css({
  overflowY: "scroll",
  padding: "20px",
  width: "90%",
  margin: "0 auto",
});

export function SidebarContainer({
  title,
  showBack,
  children,
}: PropsWithChildren<{ title: string; showBack?: boolean }>): JSX.Element {
  return (
    <div css={sidebarContainerOuterCss}>
      <div css={sidebarContainerInnerCss}>
        {showBack && <Link to="/">Back</Link>}
        <h2 css={{ marginBottom: "12px" }}>{title}</h2>
        <hr />
        <div>{children}</div>
      </div>
    </div>
  );
}

export function MapContainer({ children }: PropsWithChildren): JSX.Element {
  return <div css={mapContainerCss}>{children}</div>;
}
