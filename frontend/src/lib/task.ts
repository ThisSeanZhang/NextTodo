class Task {

  id: number | undefined;
  title: string;
  content: string;
  create_time: number;
  edit_time: number;
  deadline: undefined | number;
  status: number;
  belong_agenda_id: number;

  constructor(task: {
    id?: number,
    title?: string,
    content?: string,
    create_time?: number,
    edit_time?: number,
    deadline?: undefined | number,
    status: number,
    belong_agenda_id?: number | undefined,
  }) {
    this.id = task.id;
    this.title = task.title ?? '';
    this.content = task.content ?? '';
    this.create_time = task.create_time ?? new Date().getTime();
    this.edit_time = task.edit_time ?? new Date().getTime();
    this.deadline = task.deadline;
    this.status = task.status;
    this.belong_agenda_id = task.belong_agenda_id ?? 0;
  }

}

export default Task;
