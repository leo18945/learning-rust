drop table if exists course;

create table course(
    id int primary key,
    teacher_id int not null,
    name varchar(140) not null,
    time timestamp default now()
);

insert into course(id, teacher_id, name, time)
values
(1, 1, 'First  course', '2022-01-17 05:40:00'),
(2, 1, 'Second course', '2022-01-18 03:16:00');