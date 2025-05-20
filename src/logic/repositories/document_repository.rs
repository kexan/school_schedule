use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::PostgresConnection,
    models::document::{Document, NewDocument},
    schema::documents::{dsl::documents, teacher_id},
};

pub struct DocumentRepository;

impl DocumentRepository {
    pub fn create(
        connection: &mut PostgresConnection,
        new_document: NewDocument,
    ) -> QueryResult<Document> {
        diesel::insert_into(documents)
            .values(new_document)
            .get_result(connection)
    }

    pub fn get(connection: &mut PostgresConnection, document_id: Uuid) -> QueryResult<Document> {
        documents.find(document_id).first(connection)
    }

    pub fn get_by_teacher_id(
        connection: &mut PostgresConnection,
        id: i32,
    ) -> QueryResult<Vec<Document>> {
        documents.filter(teacher_id.eq(id)).load(connection)
    }

    pub fn delete(connection: &mut PostgresConnection, document_id: Uuid) -> QueryResult<usize> {
        diesel::delete(documents.find(document_id)).execute(connection)
    }
}
