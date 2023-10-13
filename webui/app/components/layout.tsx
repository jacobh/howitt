import { cx, css } from "@emotion/css";
import { PropsWithChildren } from "react";

export function Container({ children }: PropsWithChildren): JSX.Element {
  return <div className="grid-cols-4">{children}</div>;
}

const sidebarContainerOuterCss = cx(
  "col-span-1",
  css({ height: "100vh", overflowY: "scroll" })
);

const sidebarContainerInnerCss = css({
  overflowY: "scroll",
  padding: "20px 50px",
});

export function SidebarContainer({ children }: PropsWithChildren): JSX.Element {
  return (
    <div className={sidebarContainerOuterCss}>
      <div css={sidebarContainerInnerCss}>{children}</div>
    </div>
  );
}

const mapContainerCss = cx("col-span-3");

export function MapContainer({ children }: PropsWithChildren): JSX.Element {
  return <div className={mapContainerCss}>{children}</div>;
}
