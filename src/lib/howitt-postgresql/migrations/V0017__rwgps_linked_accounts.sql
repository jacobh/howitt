drop table user_linked_accounts;

create table user_rwgps_connections (
    id UUID PRIMARY KEY,
    user_id UUID not null references users(id),
    rwgps_user_id INTEGER not null,
    access_token VARCHAR(255) not null,
    created_at TIMESTAMPTZ not null default now(),
    updated_at TIMESTAMPTZ not null default now(),
    unique(user_id),
    unique(rwgps_user_id)
);
