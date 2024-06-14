CREATE TABLE IF NOT EXISTS questions (
    id VARCHAR (255) PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS answers (
    id VARCHAR (255) PRIMARY KEY,
    content TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    orig_q int REFERENCES questions(id)
);