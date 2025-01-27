import { useMutation } from "@apollo/client";
import { css } from "@emotion/react";
import { useRef, useState } from "react";
import { FragmentType, gql, useFragment } from "~/__generated__";

export const EditTripFragment = gql(`
  fragment editTrip on Trip {
    id
    name 
    description
  }
`);

const UPDATE_TRIP = gql(`
  mutation UpdateTrip($input: UpdateTripInput!) {
    updateTrip(input: $input) {
      trip {
        id
        name
        description
      }
    }
  }
`);

interface Props {
  trip: FragmentType<typeof EditTripFragment>;
  isOpen: boolean;
  onClose: () => void;
}

const modalStyles = css`
  padding: 2rem;
  border: 0;
  border-radius: 0.5rem;
  box-shadow: 0 0 0.5rem 0.25rem hsl(0 0% 0% / 10%);

  &::backdrop {
    background: hsl(0 0% 0% / 50%);
  }
`;

const formStyles = css`
  display: flex;
  flex-direction: column;
  gap: 1rem;
`;

const inputStyles = css`
  padding: 0.5rem;
`;

const buttonGroupStyles = css`
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1rem;
`;

export function EditTripModal({
  trip: tripFragment,
  isOpen,
  onClose,
}: Props): React.ReactElement {
  const trip = useFragment(EditTripFragment, tripFragment);
  const dialogRef = useRef<HTMLDialogElement>(null);

  const [name, setName] = useState(trip.name);
  const [description, setDescription] = useState(trip.description ?? "");

  const [updateTrip, { loading }] = useMutation(UPDATE_TRIP, {
    onCompleted: () => {
      onClose();
    },
  });

  const handleSubmit = (e: React.FormEvent): void => {
    e.preventDefault();

    updateTrip({
      variables: {
        input: {
          tripId: trip.id,
          name,
          description: description || null,
        },
      },
    });
  };

  // Show/hide modal
  if (isOpen) {
    dialogRef.current?.showModal();
  } else {
    dialogRef.current?.close();
  }

  return (
    <dialog ref={dialogRef} css={modalStyles} onClose={onClose}>
      <form onSubmit={handleSubmit} css={formStyles}>
        <div>
          <label htmlFor="name">Name</label>
          <input
            css={inputStyles}
            id="name"
            type="text"
            value={name}
            onChange={(e): void => setName(e.target.value)}
            required
          />
        </div>

        <div>
          <label htmlFor="description">Description</label>
          <textarea
            css={inputStyles}
            id="description"
            value={description}
            onChange={(e): void => setDescription(e.target.value)}
            rows={4}
          />
        </div>

        <div css={buttonGroupStyles}>
          <button type="button" onClick={onClose}>
            Cancel
          </button>
          <button type="submit" disabled={loading}>
            {loading ? "Saving..." : "Save"}
          </button>
        </div>
      </form>
    </dialog>
  );
}
