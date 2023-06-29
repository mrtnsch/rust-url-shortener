use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "urls")]
pub struct Url {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub original_url: String,
    pub short_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::Url> for Relation {
    fn to() -> RelationDef {
        unimplemented!()
    }
}

impl ActiveModelBehavior for ActiveUrl {}
impl ActiveModelBehavior for Url {}

pub fn establish_connection() -> DbConn {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    DbConn::connect(&db_url).expect("Failed to connect to the database")
}
