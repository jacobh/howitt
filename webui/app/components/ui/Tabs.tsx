import { css } from "@emotion/react";

export const tabsRootStyles = css`
  display: flex;
  flex-direction: column;
  width: 100%;
`;

export const tabsListStyles = css`
  display: flex;
  border-bottom: 1px solid #ccc;
  margin-bottom: 1rem;
`;

export const tabTriggerStyles = css`
  padding: 0.5rem 1rem;
  border: none;
  background: none;
  cursor: pointer;

  &[data-state="active"] {
    border-bottom: 2px solid #000;
  }

  &:hover {
    background-color: #f5f5f5;
  }
`;
