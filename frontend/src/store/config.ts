import { defineStore } from 'pinia'
import { ref, computed } from "vue";
import Status from '../lib/status';
import Task from "../lib/task"


export const useConfigStore = defineStore('config', () => {

  const _status = ref<Status[]>([
    new Status({
      id: 0,
      name: '积压',
      emoji: '📋',
      group: 0,
      group_index: 0
    }),
    new Status({
      id: 1,
      name: '准备去做',
      emoji: '📌',
      group: 1,
      group_index: 0
    }),
    new Status({
      id: 2,
      name: '进行中',
      emoji: '🚀',
      group: 2,
      group_index: 0
    }),
    new Status({
      id: 3,
      name: '完成',
      emoji: '🏆',
      group: 3,
      group_index: 0
    }),
    new Status({
      id: 4,
      name: '取消',
      emoji: '🚷',
      group: 4,
      group_index: 0
    })
  ]);

  const status = computed(() => _status.value);

  const default_status = computed(() => _status.value[0]);

  function GET_STATUS_BY_ID(id: number): Status {
    return _status.value.find(e => e.id === id) ?? default_status.value;
  }

  
  function GET_DEFAULT_TASK(obj: {
    belong_agenda_id?: number | undefined,
  }) : Task {
    return new Task({ status: default_status.value.id , ...obj })
  }
  
  return {
    status,
    default_status,
    GET_STATUS_BY_ID,
    GET_DEFAULT_TASK,
  };
});


export default useConfigStore;

