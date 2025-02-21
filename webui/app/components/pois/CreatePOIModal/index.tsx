import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { css } from "@emotion/react";
import { useForm } from "react-hook-form";
import { useNavigate } from "@remix-run/react";
import { gql } from "~/__generated__";
import { Modal } from "../../Modal";
import { tokens } from "~/styles/tokens";
import { PointOfInterestType } from "~/__generated__/graphql";

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
  border-radius: 4px;
`;

const coordinateContainerStyles = css`
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
`;

const errorMessageStyles = css`
  color: #dc2626;
  font-size: 0.875rem;
  margin-top: 0.5rem;
`;

const buttonGroupStyles = css`
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1rem;
`;

const selectStyles = css`
  ${inputStyles}
  height: 38px;
`;

interface FormInputs {
  name: string;
  description: string;
  longitude: number;
  latitude: number;
  pointOfInterestType: PointOfInterestType;
}

export function CreatePOIModal({ isOpen, onClose }: Props): React.ReactElement {
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<FormInputs>();

  const [createPOI, { loading }] = useMutation(CreatePointOfInterestMutation, {
    onCompleted: (data) => {
      const poi = data.createPointOfInterest.pointOfInterest;
      navigate(`/pois/${poi.slug}`);
    },
  });

  const onSubmit = (data: FormInputs): void => {
    createPOI({
      variables: {
        input: {
          name: data.name,
          description: data.description || null,
          point: [
            parseFloat(data.longitude.toString()),
            parseFloat(data.latitude.toString()),
          ],
          pointOfInterestType: data.pointOfInterestType,
        },
      },
    });
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <h2>Create Point of Interest</h2>
      <form onSubmit={handleSubmit(onSubmit)} css={formStyles}>
        <div css={formFieldStyles}>
          <label htmlFor="name">Name</label>
          <div>
            <input
              id="name"
              type="text"
              css={inputStyles}
              {...register("name", { required: "Name is required" })}
            />
            {errors.name && (
              <div css={errorMessageStyles}>{errors.name.message}</div>
            )}
          </div>
        </div>

        <div css={formFieldStyles}>
          <label htmlFor="coordinates">Coordinates</label>
          <div css={coordinateContainerStyles}>
            <div>
              <input
                id="longitude"
                type="number"
                step="any"
                placeholder="Longitude"
                css={inputStyles}
                {...register("longitude", {
                  required: "Longitude is required",
                  min: -180,
                  max: 180,
                })}
              />
              {errors.longitude && (
                <div css={errorMessageStyles}>{errors.longitude.message}</div>
              )}
            </div>
            <div>
              <input
                id="latitude"
                type="number"
                step="any"
                placeholder="Latitude"
                css={inputStyles}
                {...register("latitude", {
                  required: "Latitude is required",
                  min: -90,
                  max: 90,
                })}
              />
              {errors.latitude && (
                <div css={errorMessageStyles}>{errors.latitude.message}</div>
              )}
            </div>
          </div>
        </div>

        <div css={formFieldStyles}>
          <label htmlFor="type">Type</label>
          <div>
            <select
              id="type"
              css={selectStyles}
              {...register("pointOfInterestType", {
                required: "Type is required",
              })}
            >
              <option value="GENERIC">Generic</option>
              <option value="CAMPSITE">Campsite</option>
              <option value="WATER_SOURCE">Water Source</option>
              <option value="HUT">Hut</option>
              <option value="PUBLIC_TRANSPORT_STOP">
                Public Transport Stop
              </option>
            </select>
            {errors.pointOfInterestType && (
              <div css={errorMessageStyles}>
                {errors.pointOfInterestType.message}
              </div>
            )}
          </div>
        </div>

        <div css={formFieldStyles}>
          <label htmlFor="description">Description</label>
          <div>
            <textarea
              id="description"
              css={inputStyles}
              rows={4}
              {...register("description")}
            />
          </div>
        </div>

        <div css={buttonGroupStyles}>
          <button type="button" onClick={onClose}>
            Cancel
          </button>
          <button type="submit" disabled={loading}>
            {loading ? "Creating..." : "Create POI"}
          </button>
        </div>
      </form>
    </Modal>
  );
}
