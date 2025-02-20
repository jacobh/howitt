import type { LinksFunction, MetaDescriptor } from "@remix-run/node";
import {
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import { css, withEmotionCache } from "@emotion/react";
import { useContext, useRef, useEffect, useState } from "react";
import { ClientStyleContext } from "./styles/client.context";
import { ServerStyleContext } from "./styles/server.context";
import stylesheet from "./styles/__generated__/tailwind.css?url";
import { PrimaryMapContext } from "./components/map";
import OlMap from "ol/Map";

export const meta = (): MetaDescriptor[] => [
  {
    charset: "utf-8",
  },
  {
    title: "Howitt Plains",
  },
  {
    name: "viewport",
    content: "width=device-width, initial-scale=1, user-scalable=no",
  },
];

const bodyCss = css`
  margin: 0;
  font-family: "Hanken Grotesk", sans-serif;

  h1 {
    font-size: 1.875rem; /* 30px */
    line-height: 2.25rem; /* 36px */
  }

  h2 {
    font-size: 1.5rem; /* 24px */
    line-height: 2rem; /* 32px */
  }

  h3 {
    font-size: 1.25rem; /* 20px */
    line-height: 1.75rem; /* 28px */
  }

  h4 {
    font-size: 1.125rem; /* 18px */
    line-height: 1.75rem; /* 28px */
  }

  a {
    text-decoration: underline;
  }
`;

const mainCss = css`
  width: 100%;
  height: 100%;
  margin: 0;
`;

interface DocumentProps {
  children: React.ReactNode;
  title?: string;
}

const Document = withEmotionCache(
  ({ children, title }: DocumentProps, emotionCache) => {
    const serverStyleData = useContext(ServerStyleContext);
    const clientStyleData = useContext(ClientStyleContext);
    const reinjectStylesRef = useRef(true);

    // Only executed on client
    // When a top level ErrorBoundary or CatchBoundary are rendered,
    // the document head gets removed, so we have to create the style tags
    useEffect(() => {
      if (!reinjectStylesRef.current) {
        return;
      }
      // re-link sheet container
      emotionCache.sheet.container = document.head;

      // re-inject tags
      const tags = emotionCache.sheet.tags;
      emotionCache.sheet.flush();
      tags.forEach((tag) => {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (emotionCache.sheet as any)._insertTag(tag);
      });

      // reset cache to re-apply global styles
      clientStyleData.reset();
      // ensure we only do this once per mount
      reinjectStylesRef.current = false;
    }, [clientStyleData, emotionCache.sheet]);

    return (
      <html lang="en">
        <head>
          {title ? <title>{title}</title> : null}
          <Meta />
          <Links />
          {serverStyleData?.map(({ key, ids, css }) => (
            <style
              key={key}
              data-emotion={`${key} ${ids.join(" ")}`}
              // eslint-disable-next-line react/no-danger
              dangerouslySetInnerHTML={{ __html: css }}
            />
          ))}
        </head>
        <body css={bodyCss}>
          {children}
          <ScrollRestoration />
          <Scripts />
        </body>
      </html>
    );
  },
);

export default function App(): React.ReactNode {
  const [map, setMap] = useState<OlMap | undefined>(undefined);

  return (
    <Document>
      <PrimaryMapContext.Provider value={{ map, setMap }}>
        <main css={mainCss}>
          <Outlet />
        </main>
      </PrimaryMapContext.Provider>
    </Document>
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
    { rel: "stylesheet", href: stylesheet },
    {
      rel: "icon",
      type: "image/png",
      href: "/favicon-96x96.png",
      sizes: "96x96",
    },
    { rel: "icon", type: "image/svg+xml", href: "/favicon.svg" },
    { rel: "shortcut icon", href: "/favicon.ico" },
    {
      rel: "apple-touch-icon",
      sizes: "180x180",
      href: "/apple-touch-icon.png",
    },
    { rel: "manifest", href: "/site.webmanifest" },
  ];
};
