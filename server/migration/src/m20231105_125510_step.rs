use sea_orm_migration::prelude::*;

use crate::models::Steps;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Steps Table
        manager.create_table(
            Table::create()
                .table(Steps::Table)
                .col(
                    ColumnDef::new(Steps::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Steps::BelongTaskId).integer().null())
                .col(ColumnDef::new(Steps::Content).string().null())
                .col(ColumnDef::new(Steps::CreateTime).big_integer().not_null())
                .col(ColumnDef::new(Steps::EditTime).big_integer().null())
                .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove Steps Table
        manager
            .drop_table(Table::drop().table(Steps::Table).to_owned())
            .await
    }
}