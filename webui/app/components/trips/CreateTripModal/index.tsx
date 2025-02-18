import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { css } from "@emotion/react";
import { useCallback } from "react";
import { gql } from "~/__generated__";
import { Modal } from "../../Modal";
import { useNavigate } from "@remix-run/react";
import { RideTable } from "./components/RideTable";
import { Controller, useForm } from "react-hook-form";
import { tokens } from "~/styles/tokens";

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
  border: 1px solid ${tokens.colors.grey200};
`;

const buttonGroupStyles = css`
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1rem;
`;

const errorMessageStyles = css`
  color: #dc2626;
  font-size: 0.875rem;
  margin-top: 0.5rem;
  text-align: center;
`;

interface FormInputs {
  name: string;
  description: string;
  rideIds: string[];
}

export function CreateTripModal({
  isOpen,
  onClose,
  username,
}: Props): React.ReactElement {
  const navigate = useNavigate();
  const {
    register,
    control,
    handleSubmit,
    formState: { errors },
  } = useForm<FormInputs>({
    defaultValues: {
      name: "",
      description: "",
      rideIds: [],
    },
  });

  const [createTrip, { loading }] = useMutation(CreateTripMutation, {
    onCompleted: (data) => {
      const trip = data.createTrip.trip;
      navigate(`/riders/${trip.user.username}/trips/${trip.year}/${trip.slug}`);
    },
  });

  const onSubmit = useCallback(
    (data: FormInputs): void => {
      createTrip({
        variables: {
          input: {
            name: data.name,
            description: data.description || null,
            rideIds: data.rideIds,
          },
        },
      });
    },
    [createTrip],
  );

  // const handleRideSelectionChange = useCallback((rideIds: Set<string>) => {
  //   setSelectedRideIds(rideIds);
  // }, []);

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <form onSubmit={handleSubmit(onSubmit)} css={formStyles}>
        <div css={formFieldStyles}>
          <label htmlFor="name">Name</label>
          <input
            css={inputStyles}
            id="name"
            type="text"
            {...register("name", {
              required: "Trip name is required",
            })}
            autoComplete="off"
          />
        </div>
        {errors.name && (
          <div css={errorMessageStyles}>{errors.name.message}</div>
        )}

        <div css={formFieldStyles}>
          <label htmlFor="description">Description</label>
          <textarea
            css={inputStyles}
            id="description"
            {...register("description")}
            rows={4}
          />
        </div>

        <Controller
          name="rideIds"
          control={control}
          rules={{
            validate: (value) =>
              value.length > 0 || "Please select at least one ride",
          }}
          render={({ field: { onChange, value } }): React.ReactElement => (
            <RideTable
              username={username}
              selectedRideIds={new Set(value)}
              onSelectionChange={(rideIds): void => {
                onChange(Array.from(rideIds));
              }}
            />
          )}
        />
        {errors.rideIds && (
          <div css={errorMessageStyles}>{errors.rideIds.message}</div>
        )}

        <div css={buttonGroupStyles}>
          <button type="button" onClick={onClose}>
            Cancel
          </button>
          <button type="submit" disabled={loading}>
            {loading ? "Creating..." : "Create Trip"}
          </button>
        </div>
      </form>
    </Modal>
  );
}
