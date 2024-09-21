CREATE TABLE saved_roms
(
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER REFERENCES users (id) ON DELETE CASCADE,
    name        VARCHAR(50)                         NOT NULL,
    description TEXT,
    data        BYTEA                               NOT NULL CHECK (LENGTH(data) < 65536),
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER roms_update_trigger
    BEFORE INSERT
    ON saved_roms
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE saved_states
(
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER REFERENCES users (id) ON DELETE CASCADE,
    name        VARCHAR(50)                         NOT NULL,
    description TEXT,
    data        BYTEA                               NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER roms_update_trigger
    BEFORE INSERT
    ON saved_states
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();