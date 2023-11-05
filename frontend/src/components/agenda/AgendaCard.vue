<script setup lang="ts">
import { useRouter } from 'vue-router';
// import { CashOutline as CashIcon } from '@vicons/ionicons5'
import {
  ListChecked,
  Settings,
  Delete
} from '@vicons/carbon'
// import { computed } from 'vue';

const props = defineProps(['agenda']);

const router = useRouter();

function to_agenda_info() {
  if (props.agenda) {
    router.push(`/agendas/${props.agenda.id}/tasks`);
  }
}
// const month = 1000 * 60 * 60 * 24 * 7 * 30;
// const date_type = computed(() => {
//   const now = new Date().getTime();
//   if (now - props.agenda.create_time > month) {
//     return "datetime"
//   } else {
//     return "relative"
//   }
// })
</script>
<template>
  <n-card :title="`${props.agenda.icon} ${props.agenda.name}`">
    <template #header-extra>
      <n-space>
        <n-button text size="medium"><n-icon>
          <Delete />
        </n-icon></n-button>
        
        <n-button text size="medium"><n-icon>
          <Settings />
        </n-icon></n-button>
        
        <n-button text @click="to_agenda_info"><n-icon>
          <ListChecked />
        </n-icon></n-button>
        
      </n-space>
    </template>
    {{ props.agenda.description }}

    <template #footer>
      <n-space justify="space-between">
        <!-- tags -->
        <n-space>
            <n-tag :bordered="false" type="success">
              NextToDo
            </n-tag>
            <n-tag :bordered="false" type="warning">
              测试
            </n-tag>
            <n-tag :bordered="false" type="info">
              基础设施
            </n-tag>
            <n-tag :bordered="false" type="error">
              紧急
            </n-tag>
          </n-space>
        <!-- <n-space><n-time time-zone="Asia/Shanghai" :format="date_type === 'relative' ? '' : 'yyyy-MM-dd'" :time="props.agenda.create_time" :type="date_type" /></n-space> -->
      </n-space>
    </template>
  </n-card>
</template>

