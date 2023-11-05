use sea_orm_migration::prelude::*;

use crate::models::Tasks;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tasks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tasks::Title).string().not_null())
                    .col(ColumnDef::new(Tasks::Content).string().not_null())
                    .col(ColumnDef::new(Tasks::CreateTime).big_integer().not_null())
                    .col(ColumnDef::new(Tasks::EditTime).big_integer().not_null())
                    .col(ColumnDef::new(Tasks::Deadline).big_integer().null())
                    .col(ColumnDef::new(Tasks::Status).integer().not_null().default(0))
                    .col(ColumnDef::new(Tasks::BelongAgendaId).integer().default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await
    }
}
