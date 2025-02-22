import { useMutation } from "@apollo/client/react/hooks/useMutation";
import { css } from "@emotion/react";
import { useCallback } from "react";
import { Controller, useForm } from "react-hook-form";
import { FragmentType, gql, useFragment } from "~/__generated__";
import { Modal } from "../../Modal";
import { tokens } from "~/styles/tokens";
import { PointOfInterestType } from "~/__generated__/graphql";
import { DEFAULT_INITIAL_VIEW, Map as MapComponent } from "~/components/map";
import { Marker } from "~/components/map/types";

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
  value: { latitude: number; longitude: number };
  onChange: (value: { latitude: number; longitude: number }) => void;
}

function LocationMap({
  value,
  onChange,
}: LocationMapProps): React.ReactElement {
  const marker: Marker = {
    id: "edit-poi",
    point: [value.longitude, value.latitude],
    style: "highlighted",
  };

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
        markers={[marker]}
        initialView={DEFAULT_INITIAL_VIEW}
        onEvent={handleMapEvent}
      />
    </div>
  );
}

export function EditPOIModal({
  poi: poiFragment,

  isOpen,
  onClose,
  refetch,
}: Props): React.ReactElement {
  const poi = useFragment(EditPOIFragment, poiFragment);

  const {
    register,
    handleSubmit,
    formState: { errors },
    control,
  } = useForm<FormInputs>({
    defaultValues: {
      name: poi.name,
      description: poi.description || "",
      location: {
        latitude: poi.point[1],
        longitude: poi.point[0],
      },
      pointOfInterestType: poi.pointOfInterestType,
    },
  });

  const [updatePOI, { loading }] = useMutation(UpdatePointOfInterestMutation, {
    onCompleted: () => {
      refetch();
      onClose();
    },
  });

  const onSubmit = (data: FormInputs): void => {
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
      <h2
        css={css`
          margin-bottom: 1rem;
        `}
      >
        Edit Point of Interest
      </h2>

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
          <label htmlFor="location">Location</label>
          <div>
            <Controller
              control={control}
              name="location"
              rules={{ required: "Location is required" }}
              render={({ field: { value, onChange } }): React.ReactElement => (
                <LocationMap value={value} onChange={onChange} />
              )}
            />
            {errors.location && (
              <div css={errorMessageStyles}>{errors.location.message}</div>
            )}
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
            {loading ? "Saving..." : "Save"}
          </button>
        </div>
      </form>
    </Modal>
  );
}
