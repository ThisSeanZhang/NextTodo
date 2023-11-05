use sea_orm_migration::prelude::*;

use crate::models::TaskRelates;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(TaskRelates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TaskRelates::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TaskRelates::ParentId).integer().not_null())
                    .col(ColumnDef::new(TaskRelates::ChildId).integer().not_null())
                    .col(ColumnDef::new(TaskRelates::BelongAgendaId).integer().not_null())
                    .col(ColumnDef::new(TaskRelates::CreateTime).big_integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(TaskRelates::Table).to_owned())
            .await
    }
}


