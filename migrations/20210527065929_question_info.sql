create table question
(
    id          serial primary key,
    creator     integer references "user" not null,
    description text                      not null,
    choices     text[]                    not null,
    answer      integer[]                 not null,
    type        question_type             not null,
    draft       boolean                   not null,
    deleted     boolean                   not null default false,
    created     timestamptz               not null default current_timestamp,
    updated     timestamptz               not null default current_timestamp
);

create index on question (type);

create table vote
(
    id       serial primary key,
    voter    integer references "user"   not null,
    question integer references question not null,
    action   vote_action                 not null,
    created  timestamptz                 not null default current_timestamp,
    unique (question, voter)
);

create table apply_to
(
    question integer references question not null,
    vtuber   integer references "user"   not null,
    created  timestamptz                 not null default current_timestamp,
    unique (vtuber, question)
);
