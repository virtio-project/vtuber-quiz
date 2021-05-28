create table "user"
(
    id         serial primary key,
    username   varchar(20) not null unique,
    password   text        not null,
    challenge  varchar(10),
    blocked    boolean     not null default false,
    role       user_role   not null default 'normal'::user_role,
    reputation integer     not null default 0,
    created    timestamptz not null default current_timestamp,
    updated    timestamptz not null default current_timestamp
);

create unique index on "user"(challenge) where challenge is not null;

create table bilibili
(
    uid     integer primary key check ( uid > 0 ),
    name    varchar(20) unique        not null,
    avatar  text                      not null,
    "user"  integer references "user" not null unique,
    created timestamptz               not null default current_timestamp
);

create table following
(
    follower integer references "user" not null,
    followee integer references "user" not null check ( follower <> followee ),
    private  boolean                   not null default false,
    created  timestamptz               not null default current_timestamp,
    unique (follower, followee)
);
