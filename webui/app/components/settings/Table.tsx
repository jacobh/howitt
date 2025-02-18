import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";

export const tableContainerCss = css`
  max-height: 67vh;
  overflow: hidden;
  border: 1px solid ${tokens.colors.grey200};
`;

export const tableCss = css`
  width: 100%;
  border-collapse: separate;
  border-spacing: 0;

  th,
  td {
    padding: 10px 10px;
    text-align: left;
    border-bottom: 1px solid ${tokens.colors.grey200};
  }

  th {
    background-color: ${tokens.colors.grey50};
    font-weight: 500;
    position: sticky;
    top: 0;
    z-index: 1;
  }

  tbody {
    display: block;
    overflow-y: auto;
    max-height: calc(67vh - 41px);
  }

  thead,
  tbody tr {
    display: table;
    width: 100%;
    table-layout: fixed;
  }

  tbody tr {
    transition: background-color 0.2s;
  }

  tbody tr:hover {
    background-color: ${tokens.colors.grey50};
  }
`;
