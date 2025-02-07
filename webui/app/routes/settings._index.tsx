import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { gql } from "../__generated__/gql";
import { Container, Nav } from "~/components/layout";
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { Link } from "@remix-run/react";
import { useState } from "react";

const SettingsQuery = gql(`
  query settings {
    viewer {
      ...viewerInfo
        profile {
            id
            username
            email
        }
        rwgpsConnection {
            id
            rwgpsUserId
            createdAt
            updatedAt
        }
        rwgpsAuthRequestUrl
    }
  }
`);

const InitiateRwgpsHistorySyncMutation = gql(`
  mutation initiateRwgpsHistorySync {
    initiateRwgpsHistorySync {
      ...viewerInfo
    }
  }
`);

const pageContainerCss = css({
  maxWidth: "600px",
  width: "100%",
  margin: "0 auto",
  padding: "2rem",
});

const titleCss = css({
  marginBottom: "1rem",
});

const dividerCss = css({
  marginBottom: "1rem",
});

const fieldContainerCss = css({
  marginBottom: "1.5rem",
});

const labelCss = css({
  display: "block",
  color: tokens.colors.darkGrey,
  marginBottom: "0.5rem",
  fontWeight: 500,
});

const valueCss = css({
  display: "block",
  padding: "0.5rem",
  width: "100%",
  backgroundColor: tokens.colors.offWhite,
  border: `1px solid ${tokens.colors.lightGrey}`,
  borderRadius: "4px",
  color: tokens.colors.darkGrey,
});

const linkCss = css({
  display: "inline-block",
  color: tokens.colors.darkGrey,
  textDecoration: "none",
  marginTop: "1rem",

  "&:hover": {
    textDecoration: "underline",
  },
});

const buttonCss = css({
  display: "inline-block",
  padding: "0.5rem 1rem",
  border: `1px solid ${tokens.colors.lightGrey}`,
  borderRadius: "4px",
  backgroundColor: "white",
  cursor: "pointer",
  textDecoration: "none !important",
  "&:hover": {
    backgroundColor: tokens.colors.offWhite,
  },
  "&:disabled": {
    cursor: "not-allowed",
    opacity: 0.7,
  },
});

export default function Settings(): React.ReactElement {
  const { data } = useQuery(SettingsQuery, {});
  const [hasSynced, setHasSynced] = useState(false);
  const [initiateSync, { loading: syncing }] = useMutation(
    InitiateRwgpsHistorySyncMutation,
  );

  let viewer = data?.viewer;
  let profile = viewer?.profile;
  let rwgpsConnection = viewer?.rwgpsConnection;

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <div css={pageContainerCss}>
        <h2 css={titleCss}>Settings</h2>
        <hr css={dividerCss} />
        <div css={fieldContainerCss}>
          <label css={labelCss}>Username</label>
          <div css={valueCss}>{profile?.username}</div>
        </div>

        <div css={fieldContainerCss}>
          <label css={labelCss}>Email</label>
          <div css={valueCss}>{profile?.email}</div>
        </div>

        <h3 css={titleCss}>RWGPS Connection</h3>
        <hr css={dividerCss} />

        {rwgpsConnection ? (
          <div css={fieldContainerCss}>
            <label css={labelCss}>RWGPS User ID</label>
            <div css={valueCss}>{rwgpsConnection.rwgpsUserId}</div>
            <div
              css={css({
                fontSize: "0.9rem",
                color: tokens.colors.darkGrey,
                marginTop: "0.5rem",
                marginBottom: "1rem",
              })}
            >
              Connected on{" "}
              {new Date(rwgpsConnection.createdAt).toLocaleDateString()}
              <br />
              Last updated{" "}
              {new Date(rwgpsConnection.updatedAt).toLocaleDateString()}
            </div>
            <p>
              New rides and routes sync automatically. Your past data has
              already been synced.
              <br />
              <br />
              Re-import past data (usually not needed).
            </p>
            <button
              onClick={() => {
                initiateSync();
                setHasSynced(true);
              }}
              disabled={syncing || hasSynced}
              css={css(buttonCss, { marginTop: "8px" })}
            >
              {hasSynced ? "Sync initiated" : "Sync RWGPS History"}
            </button>
          </div>
        ) : (
          <div css={fieldContainerCss}>
            <p css={css({ marginBottom: "1rem" })}>
              Connect your Ride with GPS account to sync your routes and
              activities.
            </p>
            <a href={viewer?.rwgpsAuthRequestUrl} css={buttonCss}>
              Connect RWGPS Account
            </a>
          </div>
        )}

        <hr css={dividerCss} />

        {profile?.username && (
          <Link to={`/riders/${profile.username}`} css={linkCss}>
            View public profile →
          </Link>
        )}
      </div>
    </Container>
  );
}
