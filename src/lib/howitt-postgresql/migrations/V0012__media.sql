-- Create the media table
CREATE TABLE media (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID NOT NULL REFERENCES users(id),
    path VARCHAR(255) NOT NULL
);

-- Create junction tables for many-to-many relationships
CREATE TABLE ride_media (
    ride_id UUID NOT NULL REFERENCES rides(id),
    media_id UUID NOT NULL REFERENCES media(id),
    PRIMARY KEY (ride_id, media_id)
);

CREATE TABLE route_media (
    route_id UUID NOT NULL REFERENCES routes(id),
    media_id UUID NOT NULL REFERENCES media(id),
    PRIMARY KEY (route_id, media_id)
);

CREATE TABLE trip_media (
    trip_id UUID NOT NULL REFERENCES trips(id),
    media_id UUID NOT NULL REFERENCES media(id),
    PRIMARY KEY (trip_id, media_id)
);

CREATE TABLE poi_media (
    poi_id UUID NOT NULL REFERENCES points_of_interest(id),
    media_id UUID NOT NULL REFERENCES media(id),
    PRIMARY KEY (poi_id, media_id)
);

-- Create indexes for efficient querying
CREATE INDEX ON media (user_id, created_at DESC);
CREATE INDEX ON ride_media (ride_id);
CREATE INDEX ON ride_media (media_id);
CREATE INDEX ON route_media (route_id);
CREATE INDEX ON route_media (media_id);
CREATE INDEX ON trip_media (trip_id);
CREATE INDEX ON trip_media (media_id);
CREATE INDEX ON poi_media (poi_id);
CREATE INDEX ON poi_media (media_id);
