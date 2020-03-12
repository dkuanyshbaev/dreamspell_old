create table if not exists glyphs (
    id serial primary key,
    num int not null,
    name varchar not null,
    image varchar not null,
    preview text not null,
    description text not null
);
