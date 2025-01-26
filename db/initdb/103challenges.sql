CREATE TABLE challenges
(
    id SERIAL PRIMARY KEY,
    owner_id INTEGER REFERENCES users(id),
    requirements BYTEA,
    needs_review BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE solutions
(
    id SERIAL PRIMARY KEY,
    solver_id INTEGER REFERENCES users(id),
    challenge_id INTEGER REFERENCES challenges(id),
    program_id INTEGER REFERENCES programs(id),
    pass_requirements BOOLEAN NOT NULL,
    grade SMALLINT DEFAULT NULL
);

