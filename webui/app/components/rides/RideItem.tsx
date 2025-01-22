import { SerializedStyles, css } from "@emotion/react";
import { Link } from "@remix-run/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { COLORS } from "~/styles/theme";
import { formatDistance } from "~/services/format";

export const RideItemFragment = gql(`
    fragment rideItem on Ride {
        id
        name
        date
        distance
        user {
            username
        }
    }
`);

interface Props {
  ride: FragmentType<typeof RideItemFragment>;
  rideTitleCss?: SerializedStyles;
  titlePostfix?: string;
}

const rideItemCss = css`
  container-type: inline-size;
`;

const defaultRideTitleCss = css({
  marginBottom: "6px",
});

const titlePostfixCss = css`
  text-decoration: none;
  color: ${COLORS.darkGrey};
`;

const subtitleContainerCss = css`
  display: flex;
`;

const rideVitalsCss = css`
  flex: 1 1 auto;
`;

export function RideItem({
  ride: rideFragment,
  titlePostfix,
  rideTitleCss,
}: Props): React.ReactNode {
  const ride = useFragment(RideItemFragment, rideFragment);

  return (
    <div className="ride-item" css={rideItemCss}>
      <p className="ride-title" css={css([defaultRideTitleCss, rideTitleCss])}>
        <Link to={`/riders/${ride.user.username}/${ride.date}/`}>
          {ride.name}
        </Link>
        {titlePostfix && (
          <span css={titlePostfixCss}>&nbsp;&nbsp;{titlePostfix}</span>
        )}
      </p>
      <div css={subtitleContainerCss}>
        <div css={rideVitalsCss}>{formatDistance(ride.distance)}</div>
      </div>
    </div>
  );
}
