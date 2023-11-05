export class Position {
  x: number;
  y: number;

  constructor(obj: { x: number, y: number } | undefined) {
    if (obj === undefined) {
      this.x = 0;
      this.y = 0;
    } else {
      this.x = Math.ceil(obj.x);
      this.y = Math.ceil(obj.y);
    }
  }
}


export class GraphPosition {

  id: number | undefined;
  belong_agenda_id: number;
  config: Map<string, Position>;

  constructor(obj: {
    id?: number,
    config?: string
    belong_agenda_id?: number,
  }) {
    this.id = obj.id;
    this.config = new Map();
    this.belong_agenda_id = obj.belong_agenda_id ?? 0;
    if (obj.config) {
      const data = JSON.parse( obj.config ?? "{}");
      for (const key in data) {
        this.config.set(key, new Position(data[key])); 
      }
    }
  }

  to_commit_obj(): any {
    return {
      id : this.id,
      config: JSON.stringify(Object.fromEntries(this.config)),
      belong_agenda_id: this.belong_agenda_id
    }
  }
}

export default GraphPosition;
