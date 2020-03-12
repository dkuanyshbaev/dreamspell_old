create table if not exists users (
    id serial primary key,
    name varchar not null,
    password varchar not null,
    created_at timestamp not null default now()
);
