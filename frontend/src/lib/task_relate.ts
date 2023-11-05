class TaskRelate {

  id: number | undefined;
  parent_id: number;
  child_id: number;
  create_time: number;
  belong_agenda_id: number;

  constructor(task: {
    id?: number,
    parent_id: number,
    child_id: number,
    create_time?: number,
    belong_agenda_id: number,
  }) {
    this.id = task.id;
    this.parent_id = task.parent_id;
    this.child_id = task.child_id;
    this.create_time = task.create_time ?? new Date().getTime();
    this.belong_agenda_id = task.belong_agenda_id;
  }

}

export default TaskRelate;
