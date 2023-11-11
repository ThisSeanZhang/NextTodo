<script setup lang="ts">
import { ref, computed, watchEffect } from 'vue';
import { update_task } from "../../api/task";
import useConfigStore from "../../store/config";

const configStore = useConfigStore();

const props = defineProps(['show', 'id', "belong_agenda_id"]);
const emit = defineEmits(['update:show'])

const show = computed({
  get() {
    return props.show
  },
  set(value) {
    emit('update:show', value)
  }
})

const title = ref('创建')

const task = ref(configStore.GET_DEFAULT_TASK({ belong_agenda_id: props.belong_agenda_id }))
watchEffect(() => {
  title.value = props.id == undefined ? '创建' : '编辑';
  task.value = configStore.GET_DEFAULT_TASK({ belong_agenda_id: props.belong_agenda_id })
})

async function confirm() {
  await update_task(task.value);
  show.value = false
}

function delete_task() {
  
  show.value = false
}
</script>
<template>
  <n-modal
    v-model:show="show"
    preset="card"
    :style="{width: '1000px', height: '550px'}"
    :title="title"
    :mask-closable=false
    :bordered="false"
    :segmented="{
      content: 'soft',
      footer: 'soft'
    }"
  >
    <!-- <template #header-extra>
      噢!
    </template> -->
    <!-- {{ note }} -->
    <!-- {{ create_option }} -->
    <!-- {{ note }} -->
    <n-scrollbar style="padding-right: 20px; max-height: 319px">
  <!-- {{ props }} -->
      <n-form label-placement="top" ref="formRef" :model="task" >
        <n-grid :cols="24" :x-gap="24">

          <!-- <n-form-item-gi :span="24" label="标签">
            <n-select
              v-model:value="task.labels"
              filterable
              multiple
              tag
              @create="create_new_tag"
              @update:value="update_value"
              :render-label="renderLabel"
              :options="labelStore.label_options"
              :fallback-option="(value: any) => ({label: value.name, value})"
            />
          </n-form-item-gi> -->
          <n-form-item-gi :span="24" label="标题">
            <n-input
              v-model:value="task.title"
            />
          </n-form-item-gi>
          <n-form-item-gi :span="24" label="内容">
            <n-input
              type="textarea"
              :autosize="{
                minRows: 4,
                maxRows: 6
              }"
              v-model:value="task.content"
            />
          </n-form-item-gi>

          <n-form-item-gi :span="24" label="Deadline">
            <n-date-picker v-model:value="task.deadline" type="datetime" clearable />
        </n-form-item-gi>

      </n-grid>
      </n-form>
    </n-scrollbar>
    <template #footer>
      <n-space justify="space-between">
        <n-space>
          <n-button v-if="props.id != undefined" type="error" @click="delete_task">删除</n-button>
        </n-space>
        <n-space>
          <n-button @click="show = false">取消</n-button>
          <n-button @click="confirm" type="success">确定</n-button>
        </n-space>
      </n-space>
    </template>
  </n-modal>
</template>