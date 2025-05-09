import { css } from "@emotion/react";
import { isNotNil } from "~/services/isNotNil";
import { tokens } from "~/styles/tokens";

interface TableItem {
  name: string;
  value: React.ReactNode;
}

interface Props {
  title?: string;
  items: TableItem[];
}

const dataTableCss = css`
  width: 100%;

  td {
    border-left: 1px solid ${tokens.colors.grey200};
    padding: 5px 10%;

    &:last-child {
      border-right: 1px solid ${tokens.colors.grey200};
    }
  }

  tr {
    display: grid;
    grid-auto-flow: column;
    grid-template-columns: 1fr 1fr;

    border-top: 1px solid ${tokens.colors.grey200};

    &:last-child {
      border-bottom: 1px solid ${tokens.colors.grey200};
    }
  }
`;

const titleCss = css`
  margin-bottom: 8px;
`;

export function DataTable({ items, title }: Props): React.ReactNode {
  return (
    <div>
      {isNotNil(title) ? <p css={titleCss}>{title}</p> : null}
      <table css={dataTableCss}>
        <tbody>
          {items.map(({ name, value }) => (
            <tr key={name}>
              <td>{name}</td>
              <td>{value}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
