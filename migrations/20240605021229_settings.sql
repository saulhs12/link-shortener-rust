-- Add migration script here
create table if not exists settings
(
    id text default 'DEFAULT_SETTINGS' not null primary key,
    encrypted_global_api_key text not null
);

insert into settings(encrypted_global_api_key)
values ('ecd94e02f3a37606ef5eacc1d6daa15834fec7bd553178522c519f989eced280');