CREATE TABLE documents
(
    id               SERIAL PRIMARY KEY,
    title            TEXT NOT NULL,
    document_details JSON
);