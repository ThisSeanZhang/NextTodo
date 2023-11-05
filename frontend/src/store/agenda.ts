import { defineStore } from "pinia";
import Agenda from "../lib/agenda";
import { computed, ref, h, Component } from "vue";
import { NIcon } from 'naive-ui'
import { Settings } from '@vicons/carbon'

import { list_agendas } from "../api/agenda";

function renderIcon (icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

function gen_agenda_sub_nemu(id: number| undefined): any[] {
  return [
    {
      label: '设置',
      key: `${id}-setting`,
      icon: renderIcon(Settings)
    }
  ]
}
export const useAgendaStore = defineStore('agenda', () => {
  const _agendas = ref<Agenda []>([]);

  const agendas = computed(() => _agendas.value);

  const side_agenda_menu = computed(() => {
    return _agendas.value.map(each => ({
      label: each.name,
      key: each.id,
      data: each,
      // children: gen_agenda_sub_nemu(each.id)
    }))
  })

  async function LIST_AGENDAS(obj: {
  }) {
    let agendas = await list_agendas()
    // console.log(agendas);
    _agendas.value = agendas;
  }

  return {
    agendas,
    side_agenda_menu,
    LIST_AGENDAS,
  };
});

export default useAgendaStore;
