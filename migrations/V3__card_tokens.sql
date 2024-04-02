CREATE TABLE card_tokens (
    users_id INTEGER NOT NULL REFERENCES users(id),
    token VARCHAR(300) NOT NULL,
    PRIMARY KEY (users_id, token)
);
