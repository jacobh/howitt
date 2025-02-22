import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { css } from "@emotion/react";
import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "@remix-run/react";
import { gql } from "~/__generated__";
import { Modal } from "../../Modal";
import { tokens } from "~/styles/tokens";
import { PointOfInterestType } from "~/__generated__/graphql";
import { DEFAULT_INITIAL_VIEW, Map as MapComponent } from "~/components/map";
import { Marker } from "~/components/map/types";
import { useCallback } from "react";

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

const mapContainerStyles = css({
  height: "500px",
  marginTop: "12px",
  borderRadius: "4px",
});

interface FormInputs {
  name: string;
  description: string;
  location: {
    latitude: number;
    longitude: number;
  };
  pointOfInterestType: PointOfInterestType;
}

interface LocationMapProps {
  value?: { latitude: number; longitude: number };
  onChange: (value: { latitude: number; longitude: number }) => void;
}

function LocationMap({
  value,
  onChange,
}: LocationMapProps): React.ReactElement {
  const marker: Marker | undefined =
    value?.latitude && value?.longitude
      ? {
          id: "new-poi",
          point: [value.longitude, value.latitude],
          style: "highlighted",
        }
      : undefined;

  const handleMapEvent = useCallback(
    (event: { coords: { lat: number; lon: number } }) => {
      onChange({
        latitude: event.coords.lat,
        longitude: event.coords.lon,
      });
    },
    [onChange],
  );

  return (
    <div css={mapContainerStyles}>
      <MapComponent
        interactive={true}
        markers={marker ? [marker] : []}
        initialView={DEFAULT_INITIAL_VIEW}
        onEvent={handleMapEvent}
      />
    </div>
  );
}

export function CreatePOIModal({ isOpen, onClose }: Props): React.ReactElement {
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    formState: { errors },
    control,
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
          point: [data.location.longitude, data.location.latitude],
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
          <div>{/* label */}</div>
          <Controller
            control={control}
            name="location"
            defaultValue={undefined}
            render={({ field: { value, onChange } }): React.ReactElement => (
              <LocationMap
                value={value}
                onChange={(newValue): void => {
                  onChange(newValue);
                }}
              />
            )}
          />
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
