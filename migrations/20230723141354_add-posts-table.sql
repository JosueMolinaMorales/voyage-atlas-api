-- Add migration script here
create table posts (
    id UUID default gen_random_uuid() PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    author UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_author FOREIGN KEY (author) REFERENCES users (id)
);
