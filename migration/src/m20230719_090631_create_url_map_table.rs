use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum UrlStore {
    Table,
    Id,
    ShortUrl,
    TargetUrl,
    CreatedAt
}


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                        .table(UrlStore::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(UrlStore::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UrlStore::ShortUrl)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(ColumnDef::new(UrlStore::TargetUrl).string().not_null())
                    .col(ColumnDef::new(UrlStore::CreatedAt).timestamp().not_null().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)))
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(UrlStore::Table).to_owned())
            .await
    }
}
