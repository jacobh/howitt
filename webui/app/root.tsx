import type { LinksFunction, MetaFunction } from "@remix-run/node";
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
  font-family: "Hanken Grotesk", sans-serif;
`;

const StyledMain = styled.main`
  width: 100%;
  height: 100%;
  margin: 0;
`;

export default function App() {
  return (
    <html lang="en">
      <head>
        <Meta />
        <Links />
        {typeof document === "undefined" ? "__STYLES__" : null}
      </head>
      <StyledBody>
        <StyledMain>
          <Outlet />
        </StyledMain>
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </StyledBody>
    </html>
  );
}

export const links: LinksFunction = () => {
  return [
    { rel: "preconnect", href: "https://fonts.googleapis.com" },
    {
      rel: "preconnect",
      href: "https://fonts.gstatic.com",
      crossOrigin: "anonymous",
    },
    {
      rel: "stylesheet",
      href: "https://fonts.googleapis.com/css2?family=Hanken+Grotesk&display=swap",
    },
  ];
};
