import { ReactNode } from "react";
import { css } from "@emotion/react";

export interface TabItemProps {
  label: string;
  children: ReactNode;
  isActive?: boolean;
}

const tabPanelStyles = css`
  margin-top: 1rem;
`;

export function TabItem({
  label,
  children,
  isActive,
}: TabItemProps): React.ReactElement {
  if (!isActive) return <></>;

  return (
    <div
      role="tabpanel"
      aria-labelledby={`tab-${label.toLowerCase()}`}
      id={`panel-${label.toLowerCase()}`}
      css={tabPanelStyles}
    >
      {children}
    </div>
  );
}
