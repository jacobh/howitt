import type { MetaFunction } from "@remix-run/node";
import {
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import styled from "styled-components";

export const meta: MetaFunction = () => ({
  charset: "utf-8",
  title: "Howitt",
  viewport: "width=device-width,initial-scale=1",
});

const StyledBody = styled.body`
  margin: 0;
`

export default function App() {
  return (
    <html lang="en">
      <head>
        <Meta />
        <Links />
        {typeof document === "undefined" ? "__STYLES__" : null}
      </head>
      <StyledBody>
        <Outlet />
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </StyledBody>
    </html>
  );
}
