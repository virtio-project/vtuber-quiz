create type user_role as enum ('normal', 'vtuber');
create type question_type as enum ('true_false', 'multi_choice', 'multi_answer');
create type vote_action as enum ('up_vote', 'down_vote', 'flag_outdated', 'flag_incorrect');

create table "user"
(
    id         serial primary key,
    password   text      not null,
    blocked    boolean   not null default false,
    role       user_role not null default 'normal'::user_role,
    reputation integer   not null default 0,
    created    timestamp not null default current_timestamp,
    updated    timestamp not null default current_timestamp
);

create table bilibili
(
    uid     integer primary key check ( uid > 0 ),
    name    varchar(20) unique        not null,
    avatar  text                      not null,
    "user"  integer references "user" not null unique,
    created timestamp                 not null default current_timestamp
);

create table following
(
    follower integer references "user" not null,
    followee integer references "user" not null check ( follower <> followee ),
    private  boolean                   not null default false,
    created  timestamp                 not null default current_timestamp,
    unique (follower, followee)
);

create table question
(
    id          serial primary key,
    creator     integer references "user" not null,
    description text                      not null,
    choices     text[]                    not null,
    answer      integer[]                 not null,
    type        question_type             not null,
    draft       boolean                   not null,
    created     timestamp                 not null default current_timestamp,
    updated     timestamp                 not null default current_timestamp
);

create index on question (type);

create table vote
(
    id       serial primary key,
    voter    integer references "user"   not null,
    question integer references question not null,
    action   vote_action                 not null,
    created  timestamp                   not null default current_timestamp,
    unique (question, voter)
);

create table apply_to
(
    question integer references question not null,
    vtuber   integer references "user"   not null,
    created  timestamp                   not null default current_timestamp,
    unique (vtuber, question)
);
