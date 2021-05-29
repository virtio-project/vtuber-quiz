-- 用户类型：普通、vtuber
create type user_role as enum ('normal', 'vtuber');

create table "user"
(
    id         serial primary key,
    -- 用户名，唯一
    username   varchar(20) not null unique,
    -- 密码，argon2存储
    password   text        not null,
    -- 绑定b站用户时的挑战字符串，未创建时为空
    challenge  varchar(10),
    -- 是否封锁
    blocked    boolean     not null default false,
    -- 用户类型
    role       user_role   not null default 'normal'::user_role,
    -- 用户声望，初始为0
    reputation integer     not null default 0,
    created    timestamptz not null default current_timestamp,
    updated    timestamptz not null default current_timestamp
);

-- 挑战字符串对用户唯一（创建后）
create unique index on "user"(challenge) where challenge is not null;

create table bilibili
(
    -- b站用户uid，唯一
    uid     integer primary key check ( uid > 0 ),
    -- b站昵称
    name    varchar(20) unique        not null,
    -- 用户头像
    avatar  text                      not null,
    -- 绑定的用户
    "user"  integer references "user" not null unique,
    created timestamptz               not null default current_timestamp
);

create table following
(
    -- 用户
    follower integer references "user" not null,
    -- 被关注的用户
    followee integer references "user" not null check ( follower <> followee ),
    -- 悄悄关注
    private  boolean                   not null default false,
    created  timestamptz               not null default current_timestamp,
    unique (follower, followee)
);
