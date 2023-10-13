import { css } from "@emotion/react";
import { COLORS } from "~/styles/theme";

interface TableItem {
  name: string;
  value: any;
}

interface Props {
  items: TableItem[];
}

const dataTableCss = css`
  width: 100%;

  td {
    border-left: 1px solid ${COLORS.offWhite};
    padding: 5px 10%;

    &:last-child {
      border-right: 1px solid ${COLORS.offWhite};
    }
  }

  tr {
    display: grid;
    grid-auto-flow: column;
    grid-template-columns: 1fr 1fr;

    border-top: 1px solid ${COLORS.offWhite};

    &:last-child {
      border-bottom: 1px solid ${COLORS.offWhite};
    }
  }
`;

export function DataTable({ items }: Props): JSX.Element {
  return (
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
  );
}
