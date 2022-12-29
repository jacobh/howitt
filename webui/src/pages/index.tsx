import Head from "next/head";
import { Inter } from "@next/font/google";
import { Map } from "../components/map";
import styled from "styled-components";
import { useQuery } from "@apollo/client";
import { gql } from "../__generated__/gql";

const inter = Inter({ subsets: ["latin"] });

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
  }
`);

export default function Home() {
  const { loading, data } = useQuery(STARRED_ROUTES_QUERY);

  return (
    <>
      <Head>
        <title>Howitt</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <StyledMain>
        <Map routes={data?.starredRoutes} />
      </StyledMain>
    </>
  );
}
