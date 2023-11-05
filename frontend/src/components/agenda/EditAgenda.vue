<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { update_agenda } from "../../api/agenda";
// import useConfigStore from "../../store/config";
import Agenda from '../../lib/agenda';
import useAgendaStore from '../../store/agenda';

// const configStore = useConfigStore();
const agendaStore = useAgendaStore();

const props = defineProps(['show', 'id']);
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

const agenda = ref(new Agenda({}))
watch(() => props.id, () => {
  title.value = props.id == undefined ? '创建' : '编辑';
  agenda.value = new Agenda({})
})

async function confirm() {
  await update_agenda(agenda.value);
  await agendaStore.LIST_AGENDAS({});
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
      <n-form label-placement="top" ref="formRef" :model="agenda" >
        <n-grid :cols="24" :x-gap="24">

          <!-- <n-form-item-gi :span="24" label="标签">
            <n-select
              v-model:value="agenda.labels"
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
              v-model:value="agenda.name"
            />
          </n-form-item-gi>
          <n-form-item-gi :span="24" label="内容">
            <n-input
              type="textarea"
              :autosize="{
                minRows: 4,
                maxRows: 6
              }"
              v-model:value="agenda.description"
            />
          </n-form-item-gi>

          <!-- <n-form-item-gi :span="24" label="Deadline">
            <n-date-picker v-model:value="agenda.deadline" type="datetime" clearable />
        </n-form-item-gi> -->

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