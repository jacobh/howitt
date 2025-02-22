import { css } from "@emotion/react";
import React from "react";
import { useForm, Controller, FormProvider } from "react-hook-form";
import { tokens } from "~/styles/tokens";
import { PointOfInterestType } from "~/__generated__/graphql";
import { LocationMap } from "./components/LocationMap";
import { capitalCase } from "change-case";

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
              {Object.values(PointOfInterestType).map((type) => (
                <option key={type} value={type}>
                  {capitalCase(type)}
                </option>
              ))}
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
