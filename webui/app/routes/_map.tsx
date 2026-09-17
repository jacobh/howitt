import { useCallback, useMemo, useRef, useState } from "react";
import { Outlet, useLocation } from "react-router";
import { Container, MapContainer, Nav } from "~/components/layout";
import { PrimaryMapContext } from "~/components/map/context";
import { useMap } from "~/components/map/hooks/useMap";

export default function MapLayout(): React.ReactElement {
  const mapElementRef = useRef<HTMLDivElement>(null);
  const { map } = useMap({ mapElementRef });
  const location = useLocation();
  const [overlay, setOverlay] = useState({
    locationKey: location.key,
    isOpen: false,
  });

  // Reset on navigation before rendering children, including Back/Forward.
  // Only overlay state resets; the map and its DOM stay mounted.
  if (overlay.locationKey !== location.key) {
    setOverlay({ locationKey: location.key, isOpen: false });
  }

  const openMapOverlay = useCallback((): void => {
    setOverlay({ locationKey: location.key, isOpen: true });
  }, [location.key]);
  const closeMapOverlay = useCallback((): void => {
    setOverlay((current) => ({ ...current, isOpen: false }));
  }, []);
  const context = useMemo(
    () => ({ map, openMapOverlay }),
    [map, openMapOverlay],
  );

  return (
    <PrimaryMapContext.Provider value={context}>
      <Container>
        <Nav />
        <Outlet />
        <MapContainer
          isOverlayActive={overlay.isOpen}
          onDismissOverlay={closeMapOverlay}
        >
          <div
            ref={mapElementRef}
            data-primary-map
            style={{ width: "100%", height: "100%" }}
          />
        </MapContainer>
      </Container>
    </PrimaryMapContext.Provider>
  );
}
