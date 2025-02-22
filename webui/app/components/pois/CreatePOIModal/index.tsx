import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { css } from "@emotion/react";
import { useNavigate } from "@remix-run/react";
import { gql } from "~/__generated__";
import { Modal } from "../../Modal";
import { POIForm, FormInputs } from "../POIForm";

const CreatePointOfInterestMutation = gql(`
  mutation CreatePointOfInterest($input: CreatePointOfInterestInput!) {
    createPointOfInterest(input: $input) {
      pointOfInterest {
        id
        name
        slug
      }
    }
  }
`);

interface Props {
  isOpen: boolean;
  onClose: () => void;
}

const headingStyles = css`
  margin-bottom: 1rem;
`;

export function CreatePOIModal({ isOpen, onClose }: Props): React.ReactElement {
  const navigate = useNavigate();

  const [createPOI, { loading }] = useMutation(CreatePointOfInterestMutation, {
    onCompleted: (data) => {
      const poi = data.createPointOfInterest.pointOfInterest;
      navigate(`/pois/${poi.slug}`);
    },
  });

  const handleSubmit = (data: FormInputs): void => {
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
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <h2 css={headingStyles}>Create Point of Interest</h2>
      <POIForm onSubmit={handleSubmit} loading={loading} onCancel={onClose} />
    </Modal>
  );
}
