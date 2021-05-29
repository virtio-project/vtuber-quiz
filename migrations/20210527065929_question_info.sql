-- 题目类型：判断题、单选、多选
create type question_type as enum ('true_false', 'multi_choice', 'multi_answer');
-- 投票类型：顶、踩、标记过时、标记错误
create type vote_action as enum ('up_vote', 'down_vote', 'flag_outdated', 'flag_incorrect');
-- 题目适用的对象：vtuber自己、粉丝、路人（？有必要吗）
create type audience as enum ('vtuber', 'fan', 'passenger');

create table question
(
    id          serial primary key,
    -- 问题创建者
    creator     integer references "user" not null,
    -- 问题描述
    description text                      not null,
    -- 选项，判断题恒为['Y', 'N']
    choices     text[]                    not null,
    -- 答案，判断题为[0]或[1]，单选题为单个选项数字，多选题为所有正确的选项的数字
    answer      integer[]                 not null,
    -- 题目类型
    type        question_type             not null,
    -- 题目适用的对象
    audiences   audience[]                not null,
    -- 是否为草稿（仅自己可见）
    draft       boolean                   not null,
    -- 是否删除
    deleted     boolean                   not null default false,
    created     timestamptz               not null default current_timestamp,
    updated     timestamptz               not null default current_timestamp
);

create index on question (type);

create table vote
(
    id       serial primary key,
    -- 投票人
    voter    integer references "user"   not null,
    -- 被投票的问题
    question integer references question not null,
    -- 投票的类型
    action   vote_action                 not null,
    created  timestamptz                 not null default current_timestamp,
    unique (question, voter)
);

create table apply_to
(
    -- 问题
    question integer references question not null,
    -- 问题归属的 vtuber （一个问题可以有多个归属）
    vtuber   integer references "user"   not null,
    created  timestamptz                 not null default current_timestamp,
    unique (vtuber, question)
);
