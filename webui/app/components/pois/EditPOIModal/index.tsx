import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { css } from "@emotion/react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { Modal } from "../../Modal";
import { POIForm, FormInputs } from "../POIForm";

export const EditPOIFragment = gql(`
  fragment editPOI on PointOfInterest {
    id
    name
    description
    point
    pointOfInterestType
  }
`);

const UpdatePointOfInterestMutation = gql(`
  mutation UpdatePointOfInterest($input: UpdatePointOfInterestInput!) {
    updatePointOfInterest(input: $input) {
      pointOfInterest {
        id
        name
        description
        point
        pointOfInterestType
      }
    }
  }
`);

interface Props {
  poi: FragmentType<typeof EditPOIFragment>;
  isOpen: boolean;
  onClose: () => void;
  refetch: () => void;
}

const headingStyles = css`
  margin-bottom: 1rem;
`;

export function EditPOIModal({
  poi: poiFragment,
  isOpen,
  onClose,
  refetch,
}: Props): React.ReactElement {
  const poi = useFragment(EditPOIFragment, poiFragment);

  const [updatePOI, { loading }] = useMutation(UpdatePointOfInterestMutation, {
    onCompleted: () => {
      refetch();
      onClose();
    },
  });

  const defaultValues: FormInputs = {
    name: poi.name,
    description: poi.description || "",
    location: {
      latitude: poi.point[1],
      longitude: poi.point[0],
    },
    pointOfInterestType: poi.pointOfInterestType,
  };

  const handleSubmit = (data: FormInputs): void => {
    updatePOI({
      variables: {
        input: {
          pointOfInterestId: poi.id,
          name: data.name,
          description: data.description || null,
          point: [data.location.longitude, data.location.latitude],
          pointOfInterestType: data.pointOfInterestType,
        },
      },
    });
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <h2 css={headingStyles}>Edit Point of Interest</h2>
      <POIForm
        defaultValues={defaultValues}
        onSubmit={handleSubmit}
        loading={loading}
        onCancel={onClose}
      />
    </Modal>
  );
}
