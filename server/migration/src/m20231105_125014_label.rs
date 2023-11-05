use sea_orm_migration::prelude::*;

use crate::models::{Labels, LabelAgenda, LabelTask};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Label Table
        manager.create_table(
            Table::create()
                .table(Labels::Table)
                .col(
                    ColumnDef::new(Labels::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Labels::Title).string().not_null())
                .col(ColumnDef::new(Labels::Color).string().null())
                .col(ColumnDef::new(Labels::Emoji).string().null())
                .col(ColumnDef::new(Labels::CreateTime).big_integer().null())
                .to_owned(),
            ).await?;

        // Create LabelAgenda Table
        manager.create_table(
            Table::create()
                .table(LabelAgenda::Table)
                .col(
                    ColumnDef::new(LabelAgenda::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(LabelAgenda::LabelId).integer().not_null())
                .col(ColumnDef::new(LabelAgenda::AgendaId).integer().not_null())
                .to_owned(),
            ).await?;
        
        // Create LabelTask Table
        manager.create_table(
            Table::create()
                .table(LabelTask::Table)
                .col(
                    ColumnDef::new(LabelTask::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(LabelTask::LabelId).integer().not_null())
                .col(ColumnDef::new(LabelTask::TaskId).integer().not_null())
                .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove Label Table
        manager
        .drop_table(Table::drop().table(Labels::Table).to_owned())
        .await?;

        // Remove LabelAgenda Table
        manager
        .drop_table(Table::drop().table(LabelAgenda::Table).to_owned())
        .await?;

        // Remove LabelTask Table
        manager
        .drop_table(Table::drop().table(LabelTask::Table).to_owned())
        .await

    }
}
