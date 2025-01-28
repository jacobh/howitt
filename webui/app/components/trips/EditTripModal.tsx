import { useMutation } from "@apollo/client";
import { css } from "@emotion/react";
import { useRef, useState } from "react";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { makeMqs } from "~/styles/mediaQueries";

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

const modalStyles = makeMqs([
  css`
    padding: 5vw;
    border: 0;
    border-radius: 0.5rem;
    box-shadow: 0 0 0.5rem 0.25rem hsl(0 0% 0% / 10%);

    width: 90vw;

    &::backdrop {
      background: hsl(0 0% 0% / 50%);
    }
  `,
  css`
    padding: 4vw;
    width: 80vw;
  `,
  css`
    padding: 3vw;
    width: 70vw;
  `,
  css`
    padding: 2vw;
    width: 60vw;
  `,
  css`
    padding: 2vw;
    width: 50vw;
  `,
  css`
    padding: 2vw;
    width: 40vw;
  `,
]);

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
