import Head from "next/head";
import { Hanken_Grotesk } from "@next/font/google";
import { Map } from "../components/map";
import styled from "styled-components";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";
import { Card } from "@mui/material";
import Grid2 from "@mui/material/Unstable_Grid2";
import { useState } from "react";

const hanken = Hanken_Grotesk();

const StyledMain = styled.main`
  width: 100%;
  height: 100%;
  margin: 0;
`;

const STARRED_ROUTES_QUERY = gql(`
  query starredRoutes {
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

export default function Home() {
  const { loading, data } = useQuery(STARRED_ROUTES_QUERY);
  const [inspectedFeatures, setInspectedFeatures] = useState<any[]>([]);

  return (
    <>
      <Head>
        <title>Howitt</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div className={hanken.className}>
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
          <Map routes={data?.starredRoutes} checkpoints={data?.checkpoints} />
        </StyledMain>
      </div>
    </>
  );
}
