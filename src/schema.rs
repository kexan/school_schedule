// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "permission_role"))]
    pub struct PermissionRole;
}

diesel::table! {
    attendances (id) {
        id -> Int4,
        student_id -> Int4,
        lesson_id -> Int4,
        is_present -> Bool,
        skip_reason -> Nullable<Text>,
    }
}

diesel::table! {
    documents (id) {
        id -> Uuid,
        name -> Varchar,
        uploaded_at -> Timestamp,
        teacher_id -> Int4,
    }
}

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
    use diesel::sql_types::*;
    use super::sql_types::PermissionRole;

    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        role -> PermissionRole,
        full_name -> Nullable<Text>,
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

diesel::joinable!(attendances -> lessons (lesson_id));
diesel::joinable!(attendances -> students (student_id));
diesel::joinable!(documents -> teachers (teacher_id));
diesel::joinable!(lessons -> student_groups (student_group_id));
diesel::joinable!(student_groups -> teachers (teacher_id));
diesel::joinable!(students -> parents (parent_id));
diesel::joinable!(students -> student_groups (student_group_id));

diesel::allow_tables_to_appear_in_same_query!(
    attendances,
    documents,
    images,
    lessons,
    parents,
    student_groups,
    students,
    teachers,
    users,
    vns,
);
