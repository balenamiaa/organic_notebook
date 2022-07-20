CREATE TABLE ideas
(
    id            SERIAL PRIMARY KEY,
    document_id   INTEGER NOT NULL,
    document_page INTEGER,
    idea_text     TEXT    NOT NULL,
    idea_details  JSON,
    FOREIGN KEY (document_id) REFERENCES documents (id)
);