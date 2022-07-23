CREATE TABLE documents
(
    id               SERIAL PRIMARY KEY,
    title            TEXT NOT NULL,
    filetype VARCHAR(255) NOT NULL DEFAULT 'null'
);