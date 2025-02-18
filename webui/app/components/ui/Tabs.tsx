import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";

export const tabsRootStyles = css`
  display: flex;
  flex-direction: column;
  width: 100%;
`;

export const tabsListStyles = css`
  display: flex;
  border-bottom: 2px solid ${tokens.colors.grey200};
  margin-bottom: 1rem;
`;

export const tabTriggerStyles = css`
  padding: 0.5rem 1rem;
  border: none;
  background: none;
  cursor: pointer;
  margin-bottom: -2px;

  border-bottom: 2px solid ${tokens.colors.grey200};

  &[data-state="active"] {
    border-bottom: 2px solid ${tokens.colors.grey950};
  }

  &:hover {
    background-color: ${tokens.colors.grey50};
  }
`;
