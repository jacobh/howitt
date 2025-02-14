import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";

export const buttonStyles = css`
  background-color: white;
  border: 1px solid ${tokens.colors.lightGrey};
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9em;

  &:hover {
    background-color: ${tokens.colors.offWhite};
  }

  &:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
`;
