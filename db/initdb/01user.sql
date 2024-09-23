CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    username      VARCHAR(50)                         NOT NULL UNIQUE CHECK (LENGTH(username) >= 5 AND username !~ '[@!#\$%&*]'),
    email         VARCHAR(254)                        NOT NULL UNIQUE CHECK(email ~ '^\w+@[a-zA-Z_]+?\.[a-zA-Z]{2,3}$'),
    password_hash VARCHAR(150)                        NOT NULL,
    created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER users_update_trigger
    BEFORE INSERT
    ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE sessions
(
    id         SERIAL PRIMARY KEY,
    user_id    INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token      VARCHAR(150)                        NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expires_at TIMESTAMP NOT NULL
);