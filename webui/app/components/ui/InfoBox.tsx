import { css } from "@emotion/react";
import { PropsWithChildren } from "react";
import { tokens } from "~/styles/tokens";

const infoBoxStyles = css({
  backgroundColor: tokens.colors.grey50,
  padding: "12px 16px",
  borderRadius: "8px",
  fontSize: "14px",
  color: tokens.colors.grey600,
  marginTop: "12px",
  marginBottom: "20px",
});

export function InfoBox({ children }: PropsWithChildren): React.ReactElement {
  return <div css={infoBoxStyles}>{children}</div>;
}
