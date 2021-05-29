create extension if not exists "uuid-ossp";

create table session
(
    -- session uuid
    id uuid primary key default uuid_generate_v4(),
    -- 用户
    "user" integer references "user"   not null,
    -- session 创建时间
    created  timestamptz                 not null default current_timestamp
)
