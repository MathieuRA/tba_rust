use schema::rooms;
use diesel::associations::BelongsTo;
use crate::schema;

#[derive(Identifiable, Queryable, Clone, Debug)]
pub struct Room {
    pub id: i32,
    pub name: String
}
