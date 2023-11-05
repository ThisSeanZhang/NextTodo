<script setup lang="ts">
import { ref, computed, watchEffect } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { 
  ListChecked,
  // Thumbnail2,
  AirportLocation,
} from "@vicons/carbon"
import CardTask from '../components/task/CardTask.vue';
import useConfigStore from "../store/config";
import Task from '../lib/task';
import {
  get_task_by_id,
  query_title,
  create_task_relate,
  query_taks_relates_by_task_id,
  get_task_by_ids,
update_task,
} from '../api/task';
import TaskRelate from '../lib/task_relate';
import Status from '../lib/status';

const route = useRoute();
const router = useRouter();
function push_to(path: string) {
  router.push(path);
}
// TODO: Using async to init

const configStore = useConfigStore();

const task = ref(new Task({ status: 0 }))

const relate_taks = ref<Map<number, Task>>(new Map());

const request_info = computed(() => {
  let agenda_id, task_id;
  if (typeof route.params.agenda_id === "string") {
    agenda_id = parseInt(route.params.agenda_id)
  } else {
    agenda_id = parseInt(route.params.agenda_id[0])
  }

  if (typeof route.params.task_id === "string") {
    task_id = parseInt(route.params.task_id)
  } else {
    task_id = parseInt(route.params.task_id[0])
  }
  return {
    agenda_id,
    task_id
  }
});

const relates = ref<TaskRelate[]>([]);
const relates_split = computed(() => {
  const a = [];
  const b = [];
  for (const each of relates.value) {
    if ( each.child_id === task.value.id ) {
      if (relate_taks.value.has(each.parent_id)) {
        a.push(relate_taks.value.get(each.parent_id));
      }
    } else {
      if (relate_taks.value.has(each.child_id)) {
        b.push(relate_taks.value.get(each.child_id));
      }
      
    }
  }
  return [a, b];
})
// TODO: Using async to init
watchEffect(async ()=> {
  let data = await get_task_by_id(request_info.value.agenda_id, request_info.value.task_id);
  task.value = data;
  if ( data.id !== undefined ) {
    relates.value = await query_taks_relates_by_task_id(data.id);
    const relate_task_id = new  Set<number>();
    for (const each of relates.value) {
      relate_task_id.add(each.parent_id);
      relate_task_id.add(each.child_id);
    }
    let tasks = await get_task_by_ids(task.value.belong_agenda_id, relate_task_id);
    relate_taks.value = new Map();
    for(const each of tasks) {
      if (each.id !== undefined) {
        relate_taks.value.set(each.id, each);
      }
    }
  }
})

const searchFather = ref("");
// const searchFather = ref("");
const fatherLoadingRef = ref(false);
const fatherOptions = ref<any[]>([]);
async function handleFatherSearch(title: string) {
  fatherLoadingRef.value = true;
  try {
    const tasks = await query_title(request_info.value.agenda_id, title);
    fatherOptions.value = tasks.map(each => ({
      label: each.title,
      value: each,
      disabled: each.id === task.value.id || relates.value.filter(e => each.id === e.parent_id || each.id === e.child_id).length > 0
    }));
  } catch(e) {
    fatherOptions.value = [];
  } finally {
    fatherLoadingRef.value = false;
  }
}

async function on_select(value: Task, father: boolean) {
  console.log(value);
  searchFather.value = "";
  fatherOptions.value = [];
  if ( task.value.id !== undefined && value.id !== undefined) {
    const ids = father ? [task.value.id, value.id] : [value.id, task.value.id];
    let relate = await create_task_relate(new TaskRelate({
      parent_id: ids[0],
      child_id: ids[1],
      belong_agenda_id: task.value.belong_agenda_id,
    }));
    relates.value.push(relate)
  }
}

const task_status = computed(() => configStore.GET_STATUS_BY_ID(task.value.status));

const spin = ref(false)
async function handleSelect(value: number) {
    if (value === task.value.status) return;
    try {
        spin.value = true;
        await update_task(new Task({...task.value, status: value}));
        task.value.status = value;
    } catch (e) {
        console.log(e)
    } finally {
        spin.value = false
    }
}

function renderIcon(status: Status): any {
    return status.emoji
}
</script>
<template>
  <!-- {{ task }} -->
  <n-grid cols="2" item-responsive>
    <n-grid-item span="1">
      <n-layout>
        <n-space vertical>
          <n-descriptions label-placement="left" :column="2">
            <template #header>
              <n-space justify="space-between">
                <n-space>
                  基本信息
                </n-space>
                
                <n-space>
                  <n-button text @click="push_to(`/agendas/${task.belong_agenda_id}/graph`)">
                    <template #icon>
                    <n-icon>
                        <AirportLocation />
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
                  <n-button text @click="push_to(`/agendas/${task.belong_agenda_id}/tasks`)">
                      <template #icon>
                      <n-icon>
                          <ListChecked />
                      </n-icon>
                      </template>
                  </n-button>
                </n-space>
              </n-space>
            </template>
            <n-descriptions-item label="状态">
            <n-spin :show="spin">
            <n-dropdown trigger="click" label-field="name" key-field='id' :render-icon="renderIcon" :options="configStore.status" @select="handleSelect">
                <n-button  size="tiny">
                    <!-- TODO: 增加 Markdown 展示 -->
                    <n-ellipsis style="max-width: 70px">
                        {{ `${task_status.emoji} ${task_status.name}` }}
                    </n-ellipsis>
                </n-button>
            </n-dropdown>
            </n-spin>
            </n-descriptions-item>
            <n-descriptions-item label="创建时间">
              <!-- {{ task.create_time }} -->
              <n-time time-zone="Asia/Shanghai" :format="'yyyy-MM-dd HH:mm:ss'" :time="task.create_time" />
            </n-descriptions-item>
            <n-descriptions-item label="最近编辑时间">
              <!-- {{ task.edit_time }} -->
              <n-time time-zone="Asia/Shanghai" :format="'yyyy-MM-dd HH:mm:ss'" :time="task.edit_time" />
            </n-descriptions-item>
            <n-descriptions-item label="截至日期" :span="2">
              <n-time v-if="task.deadline" time-zone="Asia/Shanghai" :format="'yyyy-MM-dd HH:mm:ss'" :time="task.deadline" />
              <span v-else>
                {{ '无' }}
              </span>
            </n-descriptions-item>
          </n-descriptions>
          <n-input v-model:value="task.title" type="text" placeholder="基本的 Input" />
          <n-input
            v-model:value="task.content"
            type="textarea"
            placeholder="基本的 Textarea"
          />
          <n-divider title-placement="left">
            标签
          </n-divider>
          
          <n-space ></n-space>
          <!-- <n-divider title-placement="left">
            关系
          </n-divider> -->
          
          <n-grid cols="2" x-gap="12" item-responsive>
            <n-grid-item span="1">
              <n-divider dashed title-placement="left">
                依赖于
              </n-divider>
              <n-space vertical>
                <n-select
                  v-model:value="searchFather"
                  filterable
                  placeholder="搜索任务"
                  :options="fatherOptions"
                  :loading="fatherLoadingRef"
                  remote
                  @update:show="handleFatherSearch('')"
                  @search="handleFatherSearch"
                  @update:value="(value: any) => on_select(value, false)"
                />
                <n-space vertical justify="center">
                  <!-- <span v-for="relate in relates_split[0]">
                    {{ relate_taks.get(relate.parent_id) }}
                  </span> -->
                  <CardTask v-for="relate in relates_split[0]" :task="relate" ></CardTask>
                  <!-- {{ relates_split[0] }} -->
                </n-space>
              </n-space>
            </n-grid-item>
            <n-grid-item span="1">
              <n-divider dashed title-placement="left">
                被依赖
              </n-divider>
              
              <n-space vertical>
                <n-select
                  v-model:value="searchFather"
                  filterable
                  placeholder="搜索任务"
                  :options="fatherOptions"
                  :loading="fatherLoadingRef"
                  remote
                  @update:show="handleFatherSearch('')"
                  @search="handleFatherSearch"
                  @update:value="(value: any) => on_select(value, true)"
                />
                <n-space vertical justify="center">
                  <CardTask v-for="relate in relates_split[1]" :task="relate" ></CardTask>
                  
                  <!-- <span v-for="relate in relates_split[1]">
                    {{ relate }}
                  </span> -->
                  <!-- {{ relates_split[1] }} -->
                </n-space>
              </n-space>
            </n-grid-item>
          </n-grid>
        </n-space>
      </n-layout>
    </n-grid-item>
    
    <n-grid-item span="1">
      <n-divider title-placement="left">
        子任务
      </n-divider>

    </n-grid-item>
  </n-grid>
</template>
