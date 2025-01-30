create table ride_points (
    ride_id UUID references rides unique not null,
    points JSONB NOT NULL
);

insert into ride_points (ride_id, points) (
	select id, points from rides
);

alter table rides drop column points;
