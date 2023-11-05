import api from ".";
import Task from "../lib/task";
import TaskRelate from "../lib/task_relate";

export async function list_task(agenda_id: number | undefined): Promise<Task[]> {
  try {
    let path = agenda_id === undefined ? '/tasks' : `/agendas/${agenda_id}/tasks`
    let resp = await api.get(path);
    console.log(resp)
    return resp.data.map((e: any) => new Task(e));
  } catch(e) {
    console.log(e);
    return [];
  }
}

export async function query_title(agenda_id: number, title: string): Promise<Task[]> {
  try {
    let path = `/agendas/${agenda_id}/tasks/title`;
    let resp = await api.get(path, { params: { title }});
    console.log(resp)
    return resp.data.map((e: any) => new Task(e));
  } catch(e) {
    console.log(e);
    return [];
  }
}

export async function get_task_by_id(agenda_id: number, task_id: number): Promise<Task> {
  try {
    let path = `/agendas/${agenda_id}/tasks/${task_id}`;
    let resp = await api.get(path);
    console.log(resp.data);
    return new Task(resp.data);
  } catch(e) {
    throw e;
  }
}

export async function get_task_by_ids(agenda_id: number, ids: Set<number>): Promise<Task[]> {
  try {
    let path = `/agendas/${agenda_id}/tasks`;
    let resp = await api.get(path, { params: { ids } });
    console.log(resp.data);
    return resp.data.map((e: any) => new Task(e));
  } catch(e) {
    throw e;
  }
}

export async function update_task(task: Task): Promise<void> {
  let url = task.id !== undefined ? `/tasks/${task.id}` : '/tasks';
  await api.post(url, task);
}

export async function create_task_relate(relate: TaskRelate): Promise<TaskRelate> {
  let url = `/agendas/${relate.belong_agenda_id}/relates`;
  let resp = await api.post(url, relate);
  return new TaskRelate(resp.data);
}

export async function query_taks_relates_by_task_id(task_id: number): Promise<TaskRelate[]> {
  let url = `/relates`;
  let resp = await api.get(url, { params: { task_id }});
  return resp.data.map((e: any) => new TaskRelate(e));
}

export async function query_taks_relates_by_agenda_id(agenda_id: number): Promise<TaskRelate[]> {
  let url = `/relates`;
  let resp = await api.get(url, { params: { agenda_id }});
  return resp.data.map((e: any) => new TaskRelate(e));
}

