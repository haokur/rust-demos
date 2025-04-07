use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: Option<Vec<u8>>,
    pub email: Option<Vec<u8>>,
    pub create_at: Option<PrimitiveDateTime>,
    pub update_at: Option<PrimitiveDateTime>,
    pub is_delete: Option<i8>,
}
