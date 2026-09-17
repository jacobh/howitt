// @vitest-environment happy-dom
import { act, cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, test, vi } from "vitest";
import { createMemoryRouter, RouterProvider } from "react-router";
import { useContext, useState } from "react";
import MapLayout from "~/routes/_map";
import { usePrimaryMapContent } from "~/components/map/hooks/usePrimaryMapContent";
import { useMapOverlay } from "~/components/map/hooks/useMapOverlay";
import { PrimaryMapContext } from "~/components/map/context";
import { DEFAULT_INITIAL_VIEW } from "~/components/map";
import type { Track } from "~/components/map/types";
import OlMap from "ol/Map";
import MapBrowserEvent from "ol/MapBrowserEvent";

// Authentication is unrelated; keep the real layout, map and page hooks.
vi.mock("~/components/layout/Viewer", () => ({
  useViewer: () => ({ status: "anonymous" }),
}));

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

test("navigation preserves the map target and basemap, replaces page layers and handlers, and permits another layout", async () => {
  // happy-dom has no canvas renderer. Exercise real OL lifecycle and layers here;
  // actual painting is checked in Chromium against the running application.
  vi.spyOn(OlMap.prototype, "render").mockImplementation(() => {});
  let map: OlMap | undefined;
  const onClick = vi.fn();
  const tracks: Track[] = [
    {
      id: "a",
      kind: "route",
      points: [
        [140, -32],
        [143, -34],
      ],
    },
    {
      id: "b",
      kind: "route",
      points: [
        [148, -35],
        [149, -36],
      ],
    },
  ];
  function RoutesPage() {
    map = useContext(PrimaryMapContext)?.map;
    const [filtered, setFiltered] = useState(false);
    const { openMapOverlay } = useMapOverlay();
    usePrimaryMapContent({
      tracks: filtered ? [] : [...tracks, tracks[0]],
      markers: filtered
        ? []
        : [
            { id: "m1", point: [145, -36] },
            { id: "m2", point: [149, -35] },
            { id: "m1", point: [145, -36] },
          ],
      initialView: DEFAULT_INITIAL_VIEW,
      onEvent: onClick,
    });
    return (
      <>
        <button onClick={() => setFiltered(true)}>Filter</button>
        <button onClick={openMapOverlay}>Open map</button>
      </>
    );
  }
  function TripsPage() {
    map = useContext(PrimaryMapContext)?.map;
    usePrimaryMapContent({
      tracks: [{ ...tracks[0], id: "ride", kind: "ride" }],
      initialView: { type: "tracks", trackIds: ["ride"] },
    });
    return <h1>Trips</h1>;
  }
  const router = createMemoryRouter(
    [
      {
        Component: MapLayout,
        children: [
          { path: "/routes", Component: RoutesPage },
          { path: "/trips", Component: TripsPage },
        ],
      },
      { path: "/other", element: <h1>Different layout</h1> },
    ],
    { initialEntries: ["/routes"] },
  );
  const rendered = render(<RouterProvider router={router} />);
  expect(map).toBeDefined();
  if (!map) throw new Error("Map did not mount");
  const originalMap = map;
  const target = map.getTargetElement();
  const basemap = map.getLayers().item(0);
  const viewport = map.getViewport();
  const dispose = vi.spyOn(map, "dispose");
  const retarget = vi.spyOn(map, "setTarget");
  const intervals = vi.spyOn(globalThis, "setInterval");
  const listeners = map.getListeners("click") ?? [];
  expect(listeners).toHaveLength(1);
  const initialListener = listeners[0];
  expect(map.getLayers().getLength()).toBe(5);
  expect(target.parentElement?.classList.contains("overlay")).toBe(false);
  fireEvent.click(rendered.getByText("Open map"));
  expect(target.parentElement?.classList.contains("overlay")).toBe(true);
  const mask = target.parentElement?.previousElementSibling;
  if (!mask) throw new Error("Overlay mask did not mount");
  fireEvent.click(mask);
  expect(target.parentElement?.classList.contains("overlay")).toBe(false);
  fireEvent.click(rendered.getByText("Open map"));
  vi.spyOn(map, "getCoordinateFromPixel").mockReturnValue([144, -35]);
  const click = new MapBrowserEvent("click", map, new PointerEvent("click"));
  click.pixel = [200, 100];
  map.dispatchEvent(click);
  expect(onClick).toHaveBeenCalledWith({
    type: "click",
    coords: { lon: 144, lat: -35 },
  });
  onClick.mockClear();

  fireEvent.click(rendered.getByText("Filter"));
  expect(map.getLayers().getArray()).toEqual([basemap]);
  expect(intervals).not.toHaveBeenCalled();
  intervals.mockRestore();

  // Re-enter with populated layers to test cleanup on navigation too.
  await act(() => router.navigate("/trips"));
  await act(() => router.navigate("/routes"));
  expect(map.getLayers().getLength()).toBe(5);
  expect(target.parentElement?.classList.contains("overlay")).toBe(false);
  fireEvent.click(rendered.getByText("Open map"));
  map.setSize([800, 600]);
  await act(() => router.navigate("/trips"));
  expect(map).toBe(originalMap);
  expect(map.getTargetElement()).toBe(target);
  expect(map.getViewport()).toBe(viewport);
  expect(target.isConnected).toBe(true);
  expect(retarget).not.toHaveBeenCalled();
  expect(dispose).not.toHaveBeenCalled();
  expect(
    map
      .getLayers()
      .getArray()
      .map((layer) => layer.get("trackId")),
  ).toEqual([undefined, "ride"]);
  expect(map.getLayers().item(0)).toBe(basemap);
  expect(map.getListeners("click")).not.toContain(initialListener);
  expect(map.getListeners("click")).toHaveLength(1);
  map.dispatchEvent(click);
  expect(onClick).not.toHaveBeenCalled();
  expect(target.parentElement?.classList.contains("overlay")).toBe(false);
  expect(map.getView().getCenter()?.[0]).toBeCloseTo(141.5);
  expect(map.getView().getCenter()?.[1]).toBeGreaterThan(-34);
  expect(map.getView().getCenter()?.[1]).toBeLessThan(-32);
  expect(map.getView().getZoom()).toBeGreaterThan(7.5);

  // History navigation must not resurrect an overlay opened on this entry.
  await act(() => router.navigate(-1));
  expect(target.parentElement?.classList.contains("overlay")).toBe(false);
  fireEvent.click(rendered.getByText("Open map"));
  // Also reset when the page component stays mounted (e.g. query/param changes).
  await act(() => router.navigate("/routes?tags=alpine"));
  expect(target.parentElement?.classList.contains("overlay")).toBe(false);
  expect(map.getTargetElement()).toBe(target);

  await act(() => router.navigate("/other"));
  expect(rendered.getByText("Different layout")).toBeDefined();
  expect(target.isConnected).toBe(false);
  expect(dispose).toHaveBeenCalledOnce();
  expect(originalMap.getListeners("click") ?? []).toHaveLength(0);

  await act(() => router.navigate("/routes"));
  expect(map).not.toBe(originalMap);
  expect(map?.getTargetElement().isConnected).toBe(true);
  router.dispose();
});
