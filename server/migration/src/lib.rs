pub use sea_orm_migration::prelude::*;

mod models;

mod m20230724_162059_create_task;
mod m20230801_154847_create_agenda;
mod m20231013_133806_create_task_relate;
mod m20231030_151524_graph_position;
mod m20231105_125014_label;
mod m20231105_125510_step;
mod m20231105_125744_status;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230724_162059_create_task::Migration),
            Box::new(m20230801_154847_create_agenda::Migration),
            Box::new(m20231013_133806_create_task_relate::Migration),
            Box::new(m20231030_151524_graph_position::Migration),
            // Box::new(m20231105_125014_label::Migration),
            // Box::new(m20231105_125510_step::Migration),
            // Box::new(m20231105_125744_status::Migration),
        ]
    }
}
