<script setup lang="ts">
import { useRoute } from 'vue-router';
import { computed, ref, watchEffect } from 'vue';
import { 
  ModelBuilder,
  // Thumbnail2,
  CollapseCategories,
} from "@vicons/carbon"

import { list_task } from "../api/task";
import useConfigStore from "../store/config"
import EditTask from '../components/task/EditTask.vue';
// import ExhibitListItem from '../components/task/ExhibitListItem.vue'
import CardTask from '../components/task/CardTask.vue'
import Task from '../lib/task';
import TaskGarph from '../components/graph/TaskGarph.vue'

const route = useRoute();

const belong_agenda_id = computed(() => {
  if (typeof route.params.agenda_id === "string") {
    return parseInt(route.params.agenda_id)
  } else {
    return parseInt(route.params.agenda_id[0])
  }
});

const tasks = ref<Task[]>([]);
const show_task_edit = ref(false)

const configStore = useConfigStore();


enum Panel {
  List = 1,
  Graph,
}

const current_show = ref<Panel>(Panel.List);

watchEffect(async ()=> {
  let data = await list_task(belong_agenda_id.value);
  console.log(data);
  tasks.value = data;

  
  if (typeof route.params.type === "string") {
    if (route.params.type === 'tasks') {
      current_show.value = Panel.List
    } else if (route.params.type === 'graph') {
      current_show.value = Panel.Graph
    }
  }
})

// onMounted(async () => {
// })

// use to create collapse list
let tasks_map = computed(() => {
  let tasks_map = new Map<number, Task[]>();
  for (let status of configStore.status) {
    tasks_map.set(status.id, []);
  }
  for (let task of tasks.value) {
    let list = tasks_map.get(task.status);
    if (list !== undefined) {
      list.push(task)
    }
  }
  return tasks_map;
})

// TODO: expend not work
let expanded_names = ref<number[]>([]);
watchEffect(() => {
  let result = [];
  for (const [key, value] of tasks_map.value) {
    if (value.length > 0) {
      result.push(key)
    }
  }
  expanded_names.value = result
})

</script>
<template>
  <n-layout content-style="height: 100%; position: relative;">
    <n-layout content-style="padding: 20px 0px; padding-right: 15px;" position="absolute">
      <n-space justify="space-between" align="center">
        <n-space>
          <n-button @click="show_task_edit = !show_task_edit">创建任务</n-button>
          
          <!-- {{ expanded_names }} -->
        </n-space>
        <n-space>
          <n-button text @click="current_show = Panel.List">
              <template #icon>
              <n-icon>
                  <CollapseCategories />
              </n-icon>
              </template>
          </n-button>
          <!-- <n-button text>
              <template #icon>
              <n-icon>
                  <Thumbnail2 />
              </n-icon>
              </template>
          </n-button> -->
            <n-button text @click="current_show = Panel.Graph">
                <template #icon>
                <n-icon>
                    <ModelBuilder />
                </n-icon>
                </template>
            </n-button>
        </n-space>
      </n-space>
    </n-layout>
    
    <n-layout v-if="current_show === Panel.List"  position="absolute" style="top: 60px;" :native-scrollbar="false" content-style="padding: 20px 0px; overflow-x: hidden;">
      <n-space justify="center" item-style="width: 100%; padding-right: 15px;" >
        <n-collapse :default-expanded-names="expanded_names">
          <n-collapse-item
            v-for="status in configStore.status"
            :title="`${status.name} (${tasks_map.get(status.id)?.length ?? 0})`"
            :name="status.id"
            >
            <template #header-extra>
              <!-- extra button -->
            </template>
            <!-- <ExhibitListItem v-if="(tasks_map.get(status.id)?.length ?? 0) > 0" v-for="task in tasks_map.get(status.id)" :task="task"> </ExhibitListItem> -->
            <n-grid v-if="(tasks_map.get(status.id)?.length ?? 0) > 0" x-gap="12" :cols="4">
              <n-grid-item v-for="task in tasks_map.get(status.id)" >
                <CardTask :task="task"></CardTask>
              </n-grid-item>
            </n-grid>
            <n-empty  v-if="(tasks_map.get(status.id)?.length ?? 0) === 0" description="空空如也">
              <template #extra>
                <n-button size="small">
                  创建一个
                </n-button>
              </template>
            </n-empty>



          </n-collapse-item>
        </n-collapse>
      </n-space>
      
    </n-layout>
    <n-layout position="absolute" style="top: 60px;" v-else-if="current_show === Panel.Graph" >
      <TaskGarph :agenda_id="belong_agenda_id" :tasks="tasks"></TaskGarph>
    </n-layout>
    <EditTask v-model:show="show_task_edit" :belong_agenda_id="belong_agenda_id"/>
  </n-layout>
</template>
