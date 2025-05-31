CREATE TABLE IF NOT EXISTS attendances (
    id SERIAL PRIMARY KEY,
    student_id INTEGER NOT NULL REFERENCES students (id),
    lesson_id INTEGER NOT NULL REFERENCES lessons (id),
    is_present BOOLEAN NOT NULL DEFAULT false,
    skip_reason TEXT
);
