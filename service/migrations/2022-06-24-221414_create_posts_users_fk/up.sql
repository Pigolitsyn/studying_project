alter table posts
    add author_id uuid;

alter table posts
    add constraint posts_users_id_fk_2
        foreign key (author_id) references users;
