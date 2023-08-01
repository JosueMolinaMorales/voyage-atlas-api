-- Add migration script here
CREATE TABLE comments(
    id UUID NOT NULL,
    user_id UUID NOT NULL,
    post_id UUID NOT NULL,
    comment TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (post_id) REFERENCES posts (id)
);