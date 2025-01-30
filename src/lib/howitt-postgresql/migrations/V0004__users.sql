create table users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) unique not null,
    password VARCHAR(255) not null,
    email VARCHAR(255) unique not null,
    created_at TIMESTAMPTZ not null default now()
);

create table user_linked_accounts (
    user_id UUID references users(id) NOT NULL,
    vendor VARCHAR(255) not null,
    id VARCHAR(255) not null
);

create unique index on user_linked_accounts(user_id, vendor);
