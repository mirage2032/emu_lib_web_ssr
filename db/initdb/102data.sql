CREATE TABLE programs
(
    id          SERIAL PRIMARY KEY,
    owner_id    INTEGER REFERENCES users (id),
    name        VARCHAR(50)                         NOT NULL,
    description TEXT,
    data        TEXT                                NOT NULL,
    compiles    BOOLEAN                             NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER roms_update_trigger
    BEFORE INSERT
    ON programs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE states
(
    id          SERIAL PRIMARY KEY,
    owner_id    INTEGER REFERENCES users (id),
    name        VARCHAR(50)                         NOT NULL,
    description TEXT,
    data        BYTEA                               NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER roms_update_trigger
    BEFORE INSERT
    ON states
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
