import { css } from "@emotion/react";
import { PropsWithChildren } from "react";

const infoBoxStyles = css({
  backgroundColor: "#f5f5f5",
  padding: "12px 16px",
  borderRadius: "8px",
  fontSize: "14px",
  color: "#666",
  marginTop: "12px",
  marginBottom: "20px",
});

export function InfoBox({ children }: PropsWithChildren): React.ReactElement {
  return <div css={infoBoxStyles}>{children}</div>;
}
