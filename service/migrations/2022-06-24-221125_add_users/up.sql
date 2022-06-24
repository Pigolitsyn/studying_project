CREATE extension IF NOT EXISTS "uuid-ossp";

create table users
(
    id            uuid default uuid_generate_v4() not null
        constraint table_name_pk
            primary key,
    fisrtname     char(50),
    patronomic    char(50),
    surname       char(50),
    password_hash char(30),
    salt          char(10)
);

alter table users
    owner to postgres;

create unique index users_id_uindex
    on users (id);

