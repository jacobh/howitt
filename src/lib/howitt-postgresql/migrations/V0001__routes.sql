create table routes (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    name TEXT NOT NULL,
    external_ref JSONB,
    sample_points JSONB NOT NULL,
    points JSONB NOT NULL,
    distance_m integer NOT NULL,
    description text,
    published_at TIMESTAMPTZ,
    technical_difficulty VARCHAR(255),
    physical_difficulty VARCHAR(255),
    minimum_bike JSONB,
    ideal_bike JSONB,
    scouted VARCHAR(255),
    direction VARCHAR(255),
    tags VARCHAR(255)[] NOT NULL DEFAULT array[]::varchar[]
)
