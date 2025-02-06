create table route_points (
    route_id UUID references routes unique not null,
    points JSONB NOT NULL
);

insert into route_points (route_id, points) (
	select id, points from routes
);

alter table routes drop column points;
