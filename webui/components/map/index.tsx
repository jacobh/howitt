import React, { useEffect, useRef } from 'react';
import OlMap from 'ol/Map';
import View from 'ol/View';
import TileLayer from 'ol/layer/Tile';
import XYZ from 'ol/source/XYZ';
import styled from 'styled-components';
import { useGeographic } from 'ol/proj';

const MapContainer = styled.div`
    width: 100%;
    height: 100%;
    position: fixed;
`

export function Map() {
    const mapRef = useRef<OlMap>();

    useEffect(() => {
        // eslint-disable-next-line react-hooks/rules-of-hooks
        useGeographic();

        const view = new View({
            center: [147.19193300372723, -37.416399197237276],
            zoom: 7.6
          });

        const map = new OlMap({
            target: 'map',
            layers: [
              new TileLayer({
                source: new XYZ({
                  url: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png'
                })
              })
            ],
            view
        })

          mapRef.current = map

          map.addEventListener('click', (evt) => console.log(view.getCenter(), view.getZoom()))
    }, [])

    return <MapContainer id="map"/>
}