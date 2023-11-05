use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Tasks {
    Table,
    Id,
    Title,
    Content,
    CreateTime,
    EditTime,
    Deadline,
    Status,
    BelongAgendaId,
}

#[derive(Iden)]
pub enum Agendas {
    Table,
    Id,
    Icon,
    Name,
    Description,
    CreateTime,
    DefaultStatus,
}

///
#[derive(Iden)]
pub enum TaskRelates {
    Table,
    Id,
    ParentId,
    ChildId,
    BelongAgendaId,
    CreateTime,
}

#[derive(Iden)]
pub enum Labels {
    Table,
    Id,
    Title,
    Color,
    Emoji,
    CreateTime,
}

#[derive(Iden)]
pub enum LabelAgenda {
    Table,
    Id,
    LabelId,
    AgendaId,
}

#[derive(Iden)]
pub enum LabelTask {
    Table,
    Id,
    LabelId,
    TaskId,
}

#[derive(Iden)]
pub enum Steps {
    Table,
    Id,
    BelongTaskId,
    Content,
    CreateTime,
    EditTime,
}

#[derive(Iden)]
pub enum Status {
    Table,
    Id,
    Icon,
    Name,
    BelongAgendaId,
    Group,
    GroupSort,
}

