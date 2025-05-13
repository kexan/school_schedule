// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Uuid,
        image_url -> Text,
    }
}

diesel::table! {
    lessons (id) {
        id -> Int4,
        topic -> Varchar,
        scheduled_at -> Date,
        student_group_id -> Nullable<Int4>,
    }
}

diesel::table! {
    parents (id) {
        id -> Int4,
        name -> Varchar,
        additional_info -> Nullable<Text>,
    }
}

diesel::table! {
    student_groups (id) {
        id -> Int4,
        direction -> Nullable<Varchar>,
        free_spots -> Int4,
        teacher_id -> Nullable<Int4>,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        name -> Varchar,
        birth_date -> Date,
        parent_id -> Nullable<Int4>,
        student_group_id -> Nullable<Int4>,
    }
}

diesel::table! {
    teachers (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        role -> Text,
    }
}

diesel::table! {
    vns (id) {
        id -> Int4,
        title -> Text,
        alternative_titles -> Nullable<Array<Nullable<Text>>>,
        description -> Nullable<Text>,
        short_description -> Nullable<Text>,
        image_url -> Nullable<Text>,
        rating -> Nullable<Float8>,
    }
}

diesel::joinable!(lessons -> student_groups (student_group_id));
diesel::joinable!(student_groups -> teachers (teacher_id));
diesel::joinable!(students -> parents (parent_id));
diesel::joinable!(students -> student_groups (student_group_id));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    lessons,
    parents,
    student_groups,
    students,
    teachers,
    users,
    vns,
);
