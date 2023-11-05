<script setup lang="ts">
import { computed, ref } from "vue";
import { useRouter } from 'vue-router';
import { 
  Launch,
  // DataStructured,
} from "@vicons/carbon"
import useConfigStore from "../../store/config";
import Status from "../../lib/status";
import { update_task } from "../../api/task";
import Task from "../../lib/task";

const router = useRouter();
const props = defineProps(['task']);
const configStore = useConfigStore();

const spin = ref(false)
async function handleSelect(value: number) {
    if (value === props.task.status) return;
    try {
        spin.value = true;
        await update_task(new Task({...props.task, status: value}));
        props.task.status = value;
    } catch (e) {
        console.log(e)
    } finally {
        spin.value = false
    }
}

function renderIcon(status: Status): any {
    return status.emoji
}

const task_status = computed(() => configStore.GET_STATUS_BY_ID(props.task.status));

const month = 1000 * 60 * 60 * 24 * 7 * 30;
const date_type = computed(() => {
  const now = new Date().getTime();
  if (now - props.task.deadline > month) {
    return "datetime"
  } else {
    return "relative"
  }
})

function to_task_detail() {
  if (props.task) {
        router.push(`/agendas/${props.task.belong_agenda_id}/tasks/${props.task.id}`);
  }
}
</script>
<template>
    <n-card class="exhibit-card" hoverable size="small" :title="props.task.title"> 
      <!-- Content -->
      <n-ellipsis style="max-width: 80%">
        {{ props.task.content }}
      </n-ellipsis>

      <template #header-extra>
        <n-space align="center" justify="center" >
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
            <n-button text>
                <template #icon>
                <n-icon>
                    <Launch @click="to_task_detail" />
                </n-icon>
                </template>
            </n-button>
            <!-- <n-button text>
                <template #icon>
                <n-icon>
                    <Delete />
                </n-icon>
                </template>
            </n-button> -->
        </n-space>
      </template>

      <!-- Action -->
      <template #action>
        <n-space justify="space-between">
          <!-- tags -->
          <n-space>
            <n-tag :bordered="false" type="success">
              NextToDo
            </n-tag>
            <n-tag :bordered="false" type="warning">
              软件
            </n-tag>
            <n-tag :bordered="false" type="info">
              基础设施
            </n-tag>
            <n-tag :bordered="false" type="error">
              紧急
            </n-tag>
          </n-space>
          <!-- time -->
          <n-space>
            <!-- 时间展示 -->
            <n-space><n-time time-zone="Asia/Shanghai" :format="date_type === 'relative' ? '' : 'yyyy-MM-dd'" :time="props.task.create_time" :type="date_type" /></n-space>
          </n-space>
        </n-space>
      </template>
    </n-card>
</template>
<style>
.exhibit-card:hover {
    border: 1px solid var(--n-border-color);
    /* cursor: pointer; */
    border-color: #5acea7;
}
</style>
