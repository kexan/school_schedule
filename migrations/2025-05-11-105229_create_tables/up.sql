CREATE TABLE IF NOT EXISTS teachers (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS parents (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    additional_info TEXT
);

CREATE TABLE IF NOT EXISTS student_groups (
    id SERIAL PRIMARY KEY,
    direction VARCHAR NOT NULL,
    free_spots INTEGER NOT NULL,
    teacher_id INTEGER NOT NULL REFERENCES teachers (id) ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS students (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    birth_date DATE NOT NULL,
    parent_id INTEGER REFERENCES parents (id) ON DELETE SET NULL,
    student_group_id INTEGER REFERENCES student_groups (id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS lessons (
    id SERIAL PRIMARY KEY,
    topic VARCHAR NOT NULL,
    scheduled_at DATE NOT NULL,
    student_group_id INTEGER REFERENCES student_groups (id) ON DELETE SET NULL
);
