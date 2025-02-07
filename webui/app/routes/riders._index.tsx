import { DEFAULT_INITIAL_VIEW, DEFAULT_VIEW } from "../components/map";
import { useQuery } from "@apollo/client/react/hooks/useQuery";
import { gql } from "../__generated__/gql";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";
import { Link } from "@remix-run/react";
import { FragmentType, useFragment } from "~/__generated__";
import { PrimaryMap } from "~/components/map/PrimaryMap";
import { LoadingSpinnerSidebarContent } from "~/components/ui/LoadingSpinner";

const PublicUsersQuery = gql(`
  query publicUsers {
    publicUsers {
        id
        ...userItem
    }
    viewer {
      ...viewerInfo
    }
  }
`);

const userItemContainerCss = css`
  padding: 20px 1.5%;
  border-bottom: 1px solid ${tokens.colors.offWhite};

  &:hover {
    background-color: ${tokens.colors.offWhite};
  }
`;

const usernameCss = css`
  font-size: 1.25rem; /* 20px */
  line-height: 1.75rem; /* 28px */
`;

const UserItemFragment = gql(`
    fragment userItem on UserProfile {
        id
        username
    }
  `);

function UserItem(props: {
  user: FragmentType<typeof UserItemFragment>;
}): React.ReactElement {
  const user = useFragment(UserItemFragment, props.user);

  return (
    <div css={userItemContainerCss}>
      <Link css={usernameCss} to={`/riders/${user.username}`}>
        {user.username}
      </Link>
    </div>
  );
}

export default function Users(): React.ReactElement {
  const { data, loading } = useQuery(PublicUsersQuery, {});

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer titleSegments={[{ name: "Riders", linkTo: "/riders" }]}>
        {!loading ? (
          data?.publicUsers.map((user) => (
            <UserItem key={user.id} user={user} />
          ))
        ) : (
          <LoadingSpinnerSidebarContent />
        )}
      </SidebarContainer>
      <MapContainer>
        <PrimaryMap initialView={DEFAULT_INITIAL_VIEW} />
      </MapContainer>
    </Container>
  );
}
