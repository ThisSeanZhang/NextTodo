use sea_orm_migration::prelude::*;

use crate::models::Status;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Status Table
        manager.create_table(
            Table::create()
                .table(Status::Table)
                .col(
                    ColumnDef::new(Status::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Status::Icon).string().null())
                .col(ColumnDef::new(Status::Name).string().not_null())
                .col(ColumnDef::new(Status::BelongAgendaId).integer().null())
                .col(ColumnDef::new(Status::Group).integer().null())
                .col(ColumnDef::new(Status::GroupSort).integer().null())
                .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove Status Table
        manager
            .drop_table(Table::drop().table(Status::Table).to_owned())
            .await
    }
}
