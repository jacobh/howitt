import * as Accordion from "@radix-ui/react-accordion";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { useCallback, useMemo } from "react";
import { css } from "@emotion/react";
import { FormInputs, POIForm } from "~/components/pois/POIForm";
import { tokens } from "~/styles/tokens";
import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { SvgIcon } from "~/components/ui/SvgIcon";
import { chevronDownOutline } from "ionicons/icons";
import { buildRideTrack } from "~/components/map/types";
import { useQuery } from "@apollo/client/react/hooks/useQuery";

export const TripPoisFragment = gql(`
  fragment tripPois on Trip {
    id
    user {
      username
    }
  }
`);

const TripRidesQuery = gql(`
  query TripRidesForPOI($tripId: TripId!) {
    trip(id: $tripId) {
      id
      legs {
        rides {
          id
          name
          pointsJson(detailLevel: HIGH)
        }
      }
    }
  }
`);

const CreateTripPointOfInterestMutation = gql(`
  mutation CreateTripPointOfInterest($input: CreatePointOfInterestInput!) {
    createPointOfInterest(input: $input) {
      pointOfInterest {
        id
        name
        slug
      }
    }
  }
`);

const containerStyles = css`
  padding: 1rem;
  border: 1px solid ${tokens.colors.grey200};
  max-height: 80vh;
  overflow-y: auto;
`;

const accordionTriggerStyles = css`
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 0.5rem;

  .AccordionChevron {
    transition: transform 300ms;

    width: 24px;
    height: 24px;
  }

  &[data-state="open"] .AccordionChevron {
    transform: rotate(180deg);
  }
`;

const accordionContentStyles = css`
  overflow: hidden;

  &[data-state="open"] {
    animation: slideDown 300ms ease-out;
  }

  &[data-state="closed"] {
    animation: slideUp 300ms ease-out;
  }

  @keyframes slideDown {
    from {
      height: 0;
    }
    to {
      height: var(--radix-accordion-content-height);
    }
  }

  @keyframes slideUp {
    from {
      height: var(--radix-accordion-content-height);
    }
    to {
      height: 0;
    }
  }
`;

type Props = {
  trip: FragmentType<typeof TripPoisFragment>;
};

export function POITab({ trip: tripFragment }: Props): React.ReactElement {
  const trip = useFragment(TripPoisFragment, tripFragment);

  const { data: ridesData } = useQuery(TripRidesQuery, {
    variables: {
      tripId: trip.id,
    },
  });

  const tracks = useMemo(() => {
    if (!ridesData?.trip?.legs) return [];

    return ridesData.trip.legs
      .flatMap((leg) => leg.rides)
      .map((ride) => buildRideTrack(ride));
  }, [ridesData]);

  const [createPOI, { loading }] = useMutation(
    CreateTripPointOfInterestMutation,
    {
      onCompleted: () => {
        // TODO: Refresh POIs list when implemented
      },
    },
  );

  const handleSubmit = useCallback(
    (data: FormInputs): void => {
      createPOI({
        variables: {
          input: {
            name: data.name,
            description: data.description || null,
            point: [data.location.longitude, data.location.latitude],
            pointOfInterestType: data.pointOfInterestType,
          },
        },
      });
    },
    [createPOI],
  );

  return (
    <div css={containerStyles}>
      <Accordion.Root type="single" collapsible>
        <Accordion.Item value="create-poi">
          <Accordion.Header>
            <Accordion.Trigger css={accordionTriggerStyles}>
              <span>Add Point of Interest</span>
              <SvgIcon
                svgData={chevronDownOutline}
                className="AccordionChevron"
              />
            </Accordion.Trigger>
          </Accordion.Header>
          <Accordion.Content css={accordionContentStyles}>
            <POIForm
              onSubmit={handleSubmit}
              loading={loading}
              resetOnSubmit={true}
              tracks={tracks}
            />
          </Accordion.Content>
        </Accordion.Item>
      </Accordion.Root>
      {/* TODO: Add POIs list here */}
      <p>No points of interest yet</p>
    </div>
  );
}
