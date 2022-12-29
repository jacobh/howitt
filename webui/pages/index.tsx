import Head from 'next/head'
import { Inter } from '@next/font/google'
import { Map } from '../components/map'
import styled from 'styled-components'

const inter = Inter({ subsets: ['latin'] })

const StyledMain = styled.main`
  width: 100%;
  height: 100%;
  margin: 0;
`

export default function Home() {
  return (
    <>
      <Head>
        <title>Howitt</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <StyledMain>
        <Map/>
      </StyledMain>
    </>
  )
}
