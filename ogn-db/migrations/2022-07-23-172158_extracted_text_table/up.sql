CREATE TABLE extracted_texts
(
    id            SERIAL PRIMARY KEY,
    content       TEXT    NOT NULL,
    document_id   INTEGER NOT NULL,
    document_page INTEGER NOT NULL,

    FOREIGN KEY (document_id) REFERENCES documents (id),
    CONSTRAINT unique_extracted_text UNIQUE (document_id, document_page)
);