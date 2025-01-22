import { DEFAULT_VIEW, Map } from "../components/map";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import {
  Container,
  MapContainer,
  SidebarContainer,
  Nav,
} from "~/components/layout";
import { css } from "@emotion/react";
import { COLORS } from "~/styles/theme";
import { Link } from "@remix-run/react";
import { FragmentType, useFragment } from "~/__generated__";

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
  border-bottom: 1px solid ${COLORS.offWhite};

  &:hover {
    background-color: ${COLORS.offWhite};
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
  const { data } = useQuery(PublicUsersQuery, {});

  return (
    <Container>
      <Nav viewer={data?.viewer} />
      <SidebarContainer titleSegments={[{ name: "Riders", linkTo: "/riders" }]}>
        {data?.publicUsers.map((user) => (
          <UserItem key={user.id} user={user} />
        ))}
      </SidebarContainer>
      <MapContainer>
        <Map
          initialView={{
            type: "view",
            view: DEFAULT_VIEW,
          }}
        />
      </MapContainer>
    </Container>
  );
}
