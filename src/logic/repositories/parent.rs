use diesel::{QueryDsl, QueryResult, RunQueryDsl};

use crate::models::parent::UpdateParent;
use crate::schema::parents::dsl::parents;
use crate::{
    db::PostgresConnection,
    models::parent::{NewParent, Parent},
};

pub struct ParentRepository;

impl ParentRepository {
    pub fn create(
        connection: &mut PostgresConnection,
        new_parent: NewParent,
    ) -> QueryResult<Parent> {
        diesel::insert_into(parents)
            .values(new_parent)
            .get_result(connection)
    }

    pub fn get(connection: &mut PostgresConnection, parent_id: i32) -> QueryResult<Parent> {
        parents.find(parent_id).first(connection)
    }

    pub fn update(
        connection: &mut PostgresConnection,
        parent_id: i32,
        updated_parent: UpdateParent,
    ) -> QueryResult<Parent> {
        diesel::update(parents.find(parent_id))
            .set(&updated_parent)
            .get_result(connection)
    }

    pub fn delete(connection: &mut PostgresConnection, parent_id: i32) -> QueryResult<usize> {
        diesel::delete(parents.find(parent_id)).execute(connection)
    }
}
