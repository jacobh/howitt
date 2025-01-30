create table trips (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID NOT NULL REFERENCES users(id)
);

create table trip_rides (
    trip_id UUID NOT NULL REFERENCES trips(id),
    ride_id UUID NOT NULL REFERENCES rides(id),
    PRIMARY KEY (trip_id, ride_id)
);

-- Create indexes for efficient querying
create index on trips (user_id, created_at desc);
create index on trip_rides (trip_id);
create index on trip_rides (ride_id);
