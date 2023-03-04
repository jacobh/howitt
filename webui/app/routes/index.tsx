// import { Hanken_Grotesk } from "@next/font/google";
import { Map } from "../components/map";
import styled from "styled-components";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { Card } from "@mui/material";
import Grid2 from "@mui/material/Unstable_Grid2";
import { useState } from "react";

// const hanken = Hanken_Grotesk();

const StyledMain = styled.main`
  width: 100%;
  height: 100%;
  margin: 0;
`;

const HOME_QUERY = gql(`
  query homeQuery {
    starredRoutes {
      id
      name
      distance
      points
    }
    checkpoints {
      id
      name
      point
      checkpointType
    }
  }
`);

const OverlayContainer = styled.div`
  position: fixed;
  z-index: 10;
  width: 100%;
`;

export default function Index() {
  const [mode] = useState("routes");

  const { loading, data } = useQuery(HOME_QUERY);
  const [inspectedFeatures, setInspectedFeatures] = useState<any[]>([]);

  return (
    <>
      {/* <div className={hanken.className}> */}
      <div>
        <StyledMain>
          <OverlayContainer>
            <Grid2 container spacing={2}>
              <Grid2 xs={4} xsOffset={8}>
                {inspectedFeatures.length > 0 && (
                  <Card>
                    <h2>Inspect Info</h2>
                  </Card>
                )}
              </Grid2>
            </Grid2>
          </OverlayContainer>
          <Map
            routes={mode === "routes" ? data?.starredRoutes : undefined}
            checkpoints={data?.checkpoints}
          />
        </StyledMain>
      </div>
    </>
  );
}
