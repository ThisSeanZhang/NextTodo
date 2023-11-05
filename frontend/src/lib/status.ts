export default class Status {
  
  id: number;
  name: string;
  emoji: string;
  group: number;
  group_index: number;

  constructor(status: {
    id?: number;
    name?: string,
    emoji?: string,
    group?: number,
    group_index?: number
  }) {
    this.id = status.id ?? 0
    this.name = status.name ?? ''
    this.emoji = status.emoji ?? ''
    this.group = status.group ?? 0
    this.group_index = status.group_index ?? 0
  }


}