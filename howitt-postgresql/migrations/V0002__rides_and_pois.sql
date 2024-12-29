create table rides (
    id UUID PRIMARY KEY,
    name VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    external_ref JSONB,
    points JSONB NOT NULL,
    distance_m integer NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ NOT NULL
);

create table points_of_interest (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    name VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL,
    point JSONB NOT NULL
);
