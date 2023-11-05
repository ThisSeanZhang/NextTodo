use common::time::current_time_stamp;
use sea_orm_migration::{prelude::*, sea_orm::{Set, ActiveModelTrait, EntityTrait, TransactionTrait}};

use crate::models::Agendas;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Agenda Table
        manager.create_table(
            Table::create()
                .table(Agendas::Table)
                .col(
                    ColumnDef::new(Agendas::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Agendas::Icon).string().null())
                .col(ColumnDef::new(Agendas::Name).string().not_null())
                .col(ColumnDef::new(Agendas::Description).string().null())
                .col(ColumnDef::new(Agendas::CreateTime).big_integer().null())
                .col(ColumnDef::new(Agendas::DefaultStatus).integer().not_null())
                .to_owned(),
            ).await?;
        
        let db = manager.get_connection();
        // List all task
        let tasks = entity::task::Entity::find()
            .all(db)
            .await?;
        
        if tasks.len() > 0 {
            let txn = db.begin().await?;
            println!("find number of task: {}", tasks.len());
            // Create A Default Agenda
            let default_agenda = entity::agenda::ActiveModel {
                icon: Set("".to_owned()),
                name: Set("Default".to_owned()),
                description: Set("".to_owned()),
                default_status: Set(0),
                create_time: Set(current_time_stamp()),
                ..Default::default()
            }
            .save(&txn)
            .await?;
            println!("Create Default Agenda: {:?}", default_agenda);
            let agenda_id = default_agenda.id.as_ref();
            println!("Default Agenda ID: {:?}", agenda_id);

            for task in tasks.into_iter() {
                let task: entity::task::ActiveModel = task.into();
                
                println!("Before Update Task: {:?}", task);
                entity::task::ActiveModel {
                    belong_agenda_id: Set(*agenda_id),
                    ..task
                }
                .update(&txn)
                .await?;
            }
            txn.commit().await?;
        } else {
            println!("No need to update Tasks")
        }
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove Agendas Table
        manager
        .drop_table(Table::drop().table(Agendas::Table).to_owned())
        .await
    }
}

