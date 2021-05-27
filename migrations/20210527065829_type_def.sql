create type user_role as enum ('normal', 'vtuber');
create type question_type as enum ('true_false', 'multi_choice', 'multi_answer');
create type vote_action as enum ('up_vote', 'down_vote', 'flag_outdated', 'flag_incorrect');
