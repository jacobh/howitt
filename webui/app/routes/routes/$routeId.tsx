import { useQuery } from "@apollo/client";
import Grid2 from "@mui/material/Unstable_Grid2";
import { Link, useParams } from "@remix-run/react";
import styled from "styled-components";
import { formatDistance } from "~/services/formatDistance";
import { gql } from "~/__generated__";
import { Map } from "../../components/map";
import { uniq } from "lodash";

const ROUTE_QUERY = gql(`
query RouteQuery($routeId: RouteId!) {
  route(id: $routeId) {
    id
    name
    externalCanonicalUrl
    distance
    points
    description
    technicalDifficulty
    physicalDifficulty
    scouted
    direction
    minimumBike {
      tyreWidth
      frontSuspension
      rearSuspension
    }
    idealBike {
      tyreWidth
      frontSuspension
      rearSuspension
    }
  }
  viewer {
    role
  }
}
`);

const SidebarContainer = styled.div`
  overflow-y: scroll;
  padding: 20px 50px;
`;

function formatTyreWidth(mm: number): string {
  if (mm <= 50) {
    return [mm, "mm"].join("");
  }
  return [Math.round((mm / 25.4) * 100) / 100, '"'].join("");
}

function formatTyreWidths(widths?: number[]): string {
  return uniq(widths).map(formatTyreWidth).join(" ~ ");
}

function formatTravel(mm: number): string {
  if (mm === 0) {
    return "rigid";
  }
  return [mm, "mm"].join("");
}

function formatTravels(travels?: number[]): string {
  return uniq(travels).map(formatTravel).join(" ~ ");
}

export default function Route() {
  const params = useParams();

  const { data } = useQuery(ROUTE_QUERY, {
    variables: { routeId: ["ROUTE", params.routeId].join("#") },
  });

  return (
    <Grid2 container spacing={2}>
      <Grid2 xs={4} style={{ overflowY: "scroll" }}>
        <SidebarContainer>
          <Link to="/">Back</Link>
          {data?.route ? (
            <>
              <h2>{data.route.name}</h2>
              <hr />
              {formatDistance(data.route.distance)}
              {data.route.description ? <p>{data.route.description}</p> : null}
              <br />
              <p>
                <a
                  target="_blank"
                  rel="noreferrer"
                  href={data.route.externalCanonicalUrl}
                >
                  {data.route.externalCanonicalUrl}
                </a>
              </p>
              <h3>Info</h3>
              <dl>
                <dt>Technical Difficulty</dt>
                <dd>{data.route.technicalDifficulty}</dd>
                <dt>Physical Difficulty</dt>
                <dd>{data.route.technicalDifficulty}</dd>
                <dt>Scouted</dt>
                <dd>{data.route.scouted}</dd>
                <dt>Direction</dt>
                <dd>{data.route.direction}</dd>
              </dl>
              <h3>Suggested Minimum Bike</h3>
              <dl>
                <dt>Tyre Width</dt>
                <dd>{formatTyreWidths(data.route.minimumBike?.tyreWidth)}</dd>
                <dt>Front Suspension</dt>
                <dd>
                  {formatTravels(data.route.minimumBike?.frontSuspension)}
                </dd>
                <dt>Rear Suspension</dt>
                <dd>{formatTravels(data.route.minimumBike?.rearSuspension)}</dd>
              </dl>
              <h3>Suggested Ideal Bike</h3>
              <dl>
                <dt>Tyre Width</dt>
                <dd>{formatTyreWidths(data.route.idealBike?.tyreWidth)}</dd>
                <dt>Front Suspension</dt>
                <dd>{formatTravels(data.route.idealBike?.frontSuspension)}</dd>
                <dt>Rear Suspension</dt>
                <dd>{formatTravels(data.route.idealBike?.rearSuspension)}</dd>
              </dl>
            </>
          ) : (
            <></>
          )}
        </SidebarContainer>
      </Grid2>
      <Grid2 xs={8}>
        <Map
          routes={data?.route ? [data.route] : []}
          initialView={data?.route ? { routeId: data.route.id } : undefined}
        />
      </Grid2>
    </Grid2>
  );
}
