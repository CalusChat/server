create table users (
  id bigserial primary key,
  username text unique not null,
  password_hash text not null
);
