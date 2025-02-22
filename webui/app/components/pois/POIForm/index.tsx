import { css } from "@emotion/react";
import React from "react";
import { useForm, Controller, FormProvider } from "react-hook-form";
import { tokens } from "~/styles/tokens";
import { DEFAULT_INITIAL_VIEW, Map as MapComponent } from "~/components/map";
import { PointOfInterestType } from "~/__generated__/graphql";
import { Marker } from "~/components/map/types";

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

const selectStyles = css`
  ${inputStyles}
  height: 38px;
`;

const mapContainerStyles = css({
  height: "500px",
  marginTop: "12px",
  borderRadius: "4px",
});

const buttonGroupStyles = css`
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1rem;
`;

export interface FormInputs {
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
  const marker: Marker | undefined = value
    ? {
        id: "poi-marker",
        point: [value.longitude, value.latitude],
        style: "highlighted",
      }
    : undefined;

  const handleMapEvent = React.useCallback(
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

export interface POIFormProps {
  defaultValues?: Partial<FormInputs>;
  onSubmit: (data: FormInputs) => void;
  loading?: boolean;
  onCancel: () => void;
}

export function POIForm({
  defaultValues,
  onSubmit,
  loading = false,
  onCancel,
}: POIFormProps): React.ReactElement {
  const methods = useForm<FormInputs>({
    defaultValues: {
      name: "",
      description: "",
      pointOfInterestType: PointOfInterestType.Generic,
      ...defaultValues,
    },
  });

  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
  } = methods;

  return (
    <FormProvider {...methods}>
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
              rules={{ required: "Please select a location on the map" }}
              render={({ field: { value, onChange } }): React.ReactElement => (
                <LocationMap
                  value={value}
                  onChange={(newValue): void => {
                    onChange(newValue);
                  }}
                />
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
              <option value={PointOfInterestType.Generic}>Generic</option>
              <option value={PointOfInterestType.Campsite}>Campsite</option>
              <option value={PointOfInterestType.WaterSource}>
                Water Source
              </option>
              <option value={PointOfInterestType.Hut}>Hut</option>
              <option value={PointOfInterestType.PublicTransportStop}>
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
          <button type="button" onClick={onCancel}>
            Cancel
          </button>
          <button type="submit" disabled={loading}>
            {loading ? "Saving..." : "Save"}
          </button>
        </div>
      </form>
    </FormProvider>
  );
}
