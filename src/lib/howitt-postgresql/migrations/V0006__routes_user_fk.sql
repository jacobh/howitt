alter table routes add column user_id UUID references users;

update routes set user_id = '01941a60-9cfd-c166-94bb-126a6d8de5fd';

alter table routes alter column user_id set not null;
