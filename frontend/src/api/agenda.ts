import api from ".";
import Agenda from "../lib/agenda";
import GraphPosition from "../lib/graph_position";

export async function list_agendas(): Promise<Agenda[]> {
  try {
    let resp = await api.get('/agendas');
    // console.log(resp)
    return resp.data.map((e: any) => new Agenda(e));
  } catch(e) {
    console.log(e);
    return [];
  }
}

export async function update_agenda(list: Agenda): Promise<void> {
  let url = list.id !== undefined ? `/agendas/${list.id}` : '/agendas';
  await api.post(url, list);
}

export async function update_or_create_grap_position(data: GraphPosition): Promise<GraphPosition> {
  let url = data.id !== undefined ? `/graph/${data.id}` : `/graph`;
  // console.log(data.to_commit_obj());
  let resp = await api.post(url, data.to_commit_obj());
  return new GraphPosition(resp.data);
}

export async function query_grap_position_by_agenda_id(agenda_id: number | undefined): Promise<GraphPosition | undefined> {
  let url = `/graph`;
  let resp;
  // console.log('result');
  if (agenda_id !== undefined ) {
    resp = await api.get(url, { params: { agenda_id }});
  } else {
    resp = await api.get(url);
  }
  if (resp.data.length > 0) {
    return new GraphPosition(resp.data[0]);
  } else {
    return undefined;
  }
}
