import { FragmentType, gql, useFragment } from "~/__generated__";
import { useCallback, useState } from "react";
import { css } from "@emotion/react";
import { FormInputs, POIForm } from "~/components/pois/POIForm";
import { tokens } from "~/styles/tokens";
import { useMutation } from "@apollo/client/react/hooks/useMutation";

export const TripPoisFragment = gql(`
  fragment tripPois on Trip {
    id
    user {
      username
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

const headerStyles = css`
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
`;

type Props = {
  trip: FragmentType<typeof TripPoisFragment>;
};

export function POITab({ trip: tripFragment }: Props): React.ReactElement {
  const trip = useFragment(TripPoisFragment, tripFragment);
  const [showForm, setShowForm] = useState(false);

  const [createPOI, { loading }] = useMutation(
    CreateTripPointOfInterestMutation,
    {
      onCompleted: () => {
        setShowForm(false);
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

  if (!showForm) {
    return (
      <div css={containerStyles}>
        <div css={headerStyles}>
          <h3>Points of Interest</h3>
          <button type="button" onClick={(): void => setShowForm(true)}>
            Add POI
          </button>
        </div>
        {/* TODO: Add POIs list here */}
        <p>No points of interest yet</p>
      </div>
    );
  }

  return (
    <div css={containerStyles}>
      <div css={headerStyles}>
        <h3>Create Point of Interest</h3>
      </div>
      <POIForm
        onSubmit={handleSubmit}
        loading={loading}
        onCancel={(): void => setShowForm(false)}
      />
    </div>
  );
}
