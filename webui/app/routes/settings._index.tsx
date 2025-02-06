import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "../__generated__/gql";
import { Container, Nav } from "~/components/layout";
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { Link } from "@remix-run/react";

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

export default function Settings(): React.ReactElement {
  const { data } = useQuery(SettingsQuery, {});

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
              })}
            >
              Connected on{" "}
              {new Date(rwgpsConnection.createdAt).toLocaleDateString()}
              <br />
              Last updated{" "}
              {new Date(rwgpsConnection.updatedAt).toLocaleDateString()}
            </div>
          </div>
        ) : (
          <div css={fieldContainerCss}>
            <p css={css({ marginBottom: "1rem" })}>
              Connect your Ride with GPS account to sync your routes and
              activities.
            </p>
            <a
              href={viewer?.rwgpsAuthRequestUrl}
              css={css({
                display: "inline-block",
                padding: "0.5rem 1rem",
                border: `1px solid #888`,
                borderRadius: "4px",
                textDecoration: "none !important",
              })}
            >
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
