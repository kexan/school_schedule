CREATE TABLE IF NOT EXISTS documents (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    uploaded_at TIME NOT NULL,
    teacher_id INTEGER NOT NULL REFERENCES teachers (id) ON DELETE CASCADE
);
