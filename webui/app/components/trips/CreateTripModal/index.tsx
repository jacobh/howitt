import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { css } from "@emotion/react";
import { useCallback, useState } from "react";
import { gql } from "~/__generated__";
import { Modal } from "../../Modal";
import { useNavigate } from "@remix-run/react";
import { RideTable } from "./components/RideTable";

const CreateTripMutation = gql(`
  mutation CreateTrip($input: CreateTripInput!) {
    createTrip(input: $input) {
      trip {
        id
        name
        slug
        year
        user {
          username
        }
      }
    }
  }
`);

interface Props {
  isOpen: boolean;
  onClose: () => void;
  username: string;
}

const formStyles = css`
  display: flex;
  flex-direction: column;
  gap: 1rem;
`;

const formFieldStyles = css`
  display: grid;
  grid-template-columns: minmax(75px, 6vw) 1fr;
  gap: 1rem;
  align-items: start;

  label {
    padding-top: 0.5rem;
  }
`;

const inputStyles = css`
  padding: 0.5rem;
  width: 100%;
  border: 1px solid #ccc;
`;

const buttonGroupStyles = css`
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1rem;
`;

export function CreateTripModal({
  isOpen,
  onClose,
  username,
}: Props): React.ReactElement {
  const navigate = useNavigate();
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [selectedRideIds, setSelectedRideIds] = useState<Set<string>>(
    new Set(),
  );

  const [createTrip, { loading }] = useMutation(CreateTripMutation, {
    onCompleted: (data) => {
      const trip = data.createTrip.trip;
      navigate(`/riders/${trip.user.username}/trips/${trip.year}/${trip.slug}`);
    },
  });

  const handleSubmit = useCallback(
    (e: React.FormEvent): void => {
      e.preventDefault();

      createTrip({
        variables: {
          input: {
            name,
            description: description || null,
            rideIds: Array.from(selectedRideIds),
          },
        },
      });
    },
    [createTrip, name, description, selectedRideIds],
  );

  const handleRideSelectionChange = useCallback((rideIds: Set<string>) => {
    setSelectedRideIds(rideIds);
  }, []);

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <form onSubmit={handleSubmit} css={formStyles}>
        <div css={formFieldStyles}>
          <label htmlFor="name">Name</label>
          <input
            css={inputStyles}
            id="name"
            type="text"
            value={name}
            onChange={(e): void => setName(e.target.value)}
            autoComplete="off"
            required
          />
        </div>

        <div css={formFieldStyles}>
          <label htmlFor="description">Description</label>
          <textarea
            css={inputStyles}
            id="description"
            value={description}
            onChange={(e): void => setDescription(e.target.value)}
            rows={4}
          />
        </div>

        <RideTable
          username={username}
          selectedRideIds={selectedRideIds}
          onSelectionChange={handleRideSelectionChange}
        />

        <div css={buttonGroupStyles}>
          <button type="button" onClick={onClose}>
            Cancel
          </button>
          <button
            type="submit"
            disabled={loading || selectedRideIds.size === 0}
          >
            {loading ? "Creating..." : "Create Trip"}
          </button>
        </div>
      </form>
    </Modal>
  );
}
