import { ReactElement, useState } from "react";
import { css } from "@emotion/react";
import { TabItem, TabItemProps } from "./TabItem";

interface TabListProps {
  children: ReactElement<TabItemProps> | ReactElement<TabItemProps>[];
  defaultTab?: number;
}

const tabListStyles = css`
  border-bottom: 1px solid #eee;
  margin-bottom: 1rem;
`;

const tabButtonStyles = css`
  padding: 0.5rem 1rem;
  border: none;
  background: none;
  cursor: pointer;

  &[aria-selected="true"] {
    border-bottom: 2px solid #000;
  }
`;

export function TabList({
  children,
  defaultTab = 0,
}: TabListProps): React.ReactElement {
  const [activeTab, setActiveTab] = useState(defaultTab);
  const tabs = Array.isArray(children) ? children : [children];

  const handleTabClick = (e: React.MouseEvent, index: number): void => {
    e.preventDefault();
    e.stopPropagation();
    setActiveTab(index);
  };

  return (
    <div>
      <div css={tabListStyles} role="tablist">
        {tabs.map((tab, index) => (
          <button
            key={index}
            role="tab"
            id={`tab-${tab.props.label.toLowerCase()}`}
            aria-controls={`panel-${tab.props.label.toLowerCase()}`}
            aria-selected={activeTab === index}
            onClick={(e): void => handleTabClick(e, index)}
            css={tabButtonStyles}
          >
            {tab.props.label}
          </button>
        ))}
      </div>
      {tabs.map((tab, index) => (
        <TabItem
          key={index}
          label={tab.props.label}
          isActive={activeTab === index}
        >
          {tab.props.children}
        </TabItem>
      ))}
    </div>
  );
}
