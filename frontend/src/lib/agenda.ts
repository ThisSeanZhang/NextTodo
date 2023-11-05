class Agenda {

  id: number | undefined;
  icon: string;
  name: string;
  description: string;
  create_time: number;
  default_status: undefined | number;

  constructor(agenda: {
    id?: number,
    icon?: string,
    name?: string,
    create_time?: number,
    description?: string,
    default_status?: number,
  }) {
    this.id = agenda.id;
    // this.icon = agenda.icon ?? '';
    this.icon = agenda.icon ?? '🐱‍👤';
    this.name = agenda.name ?? '';
    this.create_time = agenda.create_time ?? new Date().getTime();
    this.description = agenda.description ?? '';
    this.default_status = agenda.default_status ?? 0;
  }

}

export default Agenda;
