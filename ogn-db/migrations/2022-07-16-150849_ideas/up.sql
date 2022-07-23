CREATE TABLE ideas
(
    id    SERIAL PRIMARY KEY,
    label TEXT NOT NULL
        CONSTRAINT _UNIQUE_IDEA UNIQUE
);

CREATE TABLE idea_refs
(
    id            SERIAL PRIMARY KEY,
    document_id   INTEGER NOT NULL,
    document_page INTEGER,
    idea_ref      INTEGER NOT NULL,
    idea_ref_text TEXT    NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents (id),
    FOREIGN KEY (idea_ref) REFERENCES ideas (id)
);