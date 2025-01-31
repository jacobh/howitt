import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { Container, Nav } from "~/components/layout";
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { Link } from "@remix-run/react";

const SettingsQuery = gql(`
  query settings {
    viewer {
        profile {
            id
            username
            email
        }
      ...viewerInfo
    }
  }
`);

const pageContainerCss = css({
  maxWidth: "600px",
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

        {profile?.username && (
          <Link to={`/riders/${profile.username}`} css={linkCss}>
            View public profile →
          </Link>
        )}
      </div>
    </Container>
  );
}
