<script setup lang="ts">
import { Delete } from "@vicons/carbon"
import useConfigStore from "../../store/config";
import Status from "../../lib/status";
import { computed, ref } from "vue";
import { update_task } from "../../api/task";
import Task from "../../lib/task";
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
</script>
<template>
    <n-card class="exhibit-card" hoverable content-style="padding: 5px" > 
    <n-space justify="space-between" align="center">
        <!-- Info -->
        <n-space align="center" justify="center" :size='[20, 0]' >
            
            <!-- Status -->
            <n-space style="width: 80px;" align="center" justify="center" >
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
            </n-space>

           <n-space vertical style="max-width: 800px;min-width: 800px;">
                <!-- title -->
                <n-h6 prefix="bar" align-text>
                    {{ props.task.title }}
                </n-h6>
                <!-- Content -->
                <n-ellipsis style="max-width: 80%">
                    {{ props.task.content }}
                </n-ellipsis>
           </n-space>
        </n-space>
        
        <!-- Action -->
        <n-space align="center" justify="center" >
            <n-button text>
                <template #icon>
                <n-icon>
                    <Delete />
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
    </n-space>
    </n-card>
</template>
<style>
.exhibit-card:hover {
    border: 1px solid var(--n-border-color);
    cursor: pointer;
    border-color: #5acea7;
}
</style>
