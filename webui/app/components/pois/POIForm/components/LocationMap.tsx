import { css } from "@emotion/react";
import React from "react";
import { DEFAULT_INITIAL_VIEW, Map as MapComponent } from "~/components/map";
import { Marker, Track } from "~/components/map/types";

const mapContainerStyles = css({
  height: "500px",
  marginTop: "12px",
  borderRadius: "4px",
});

interface LocationMapProps {
  value?: { latitude: number; longitude: number };
  onChange: (value: { latitude: number; longitude: number }) => void;
  tracks?: Track[];
}

export function LocationMap({
  value,
  onChange,
  tracks = [],
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
        tracks={tracks}
        initialView={DEFAULT_INITIAL_VIEW}
        onEvent={handleMapEvent}
      />
    </div>
  );
}
