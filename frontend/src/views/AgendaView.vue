<script setup lang="ts">
import { PlusOutlined } from '@vicons/material'
import { onMounted, ref } from 'vue';
// import { list_agendas } from '../api/agenda'
// import Agenda from '../lib/agenda';
import AgendaCard from '../components/agenda/AgendaCard.vue';
import EditAgenda from '../components/agenda/EditAgenda.vue';
import useAgendaStore from '../store/agenda'

const agendaStore = useAgendaStore();
// const agendas = ref<Agenda[]>([]);
const show_agenda_edit = ref(false);
onMounted(async () => {
  await agendaStore.LIST_AGENDAS({});
})
</script>

<template>
<n-layout>
  <n-grid x-gap="12" y-gap="12" cols="1 400:1 812:2 1236:3 1624:4 1920:4" >
    <n-grid-item >
        <n-card hoverable class="agenda-card agenda-card-add" @click="show_agenda_edit = true">
          <n-empty description="添加一个">
            <template #icon>
              <n-icon>
                <PlusOutlined />
              </n-icon>
            </template>
          </n-empty>
        </n-card>
    </n-grid-item>
    <n-grid-item v-for="agenda in agendaStore.agendas">
      <AgendaCard class="agenda-card" :agenda="agenda"></AgendaCard>
    </n-grid-item>
  </n-grid>
  <EditAgenda v-model:show="show_agenda_edit" />
</n-layout>
</template>
<style scoped>
.agenda-card {
  min-width: 400px;
}
.agenda-card-add {
  cursor: pointer;
}
.agenda-card:hover {
  border-color: #5acea7;
}
</style>
