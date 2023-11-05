<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NIcon } from 'naive-ui'
import { h, ref, Component, onMounted } from 'vue'
import {
  BookOutline as BookIcon,
} from '@vicons/ionicons5'

import useAgendaStore from '../store/agenda'
import Agenda from '../lib/agenda';

const router = useRouter();
// const route = useRoute();

const agendaStore = useAgendaStore();

onMounted(async () => {
  await agendaStore.LIST_AGENDAS({});
})
function renderIcon (icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

// function to_tasks() {
//   router.push("/tasks")
// }

const headMenuKey = ref("")
const agendaMenuKey = ref("")
const collapsed = ref<boolean>(false)
const headMenu = ref([
  {
    label: '总览',
    key: 'dashboard',
    disabled: true,
    icon: renderIcon(BookIcon)
  },
  {
    label: '进度分析',
    key: 'analytics',
    disabled: true,
    icon: renderIcon(BookIcon)
  },
  {
    label: '待办列表',
    key: 'agenda',
    icon: renderIcon(BookIcon)
  },
  {
    label: '设置',
    key: 'setting',
    disabled: true,
    icon: renderIcon(BookIcon)
  }
])
// const menuOptions = ref([])

function click_side_menu(key: string) {
  agendaMenuKey.value = "";
  if (key === 'agenda') {
    router.push("/agendas");
  }
  // console.log(key)
}

function click_side_agenda(key: string) {
  headMenuKey.value = "";
  router.push(`/agendas/${key}/tasks`);
}

function agenda_icon_render(agenda: { icon: any, data: Agenda}) : any {

  // 渲染图标占位符以保持缩进
  // if (option.key === 'sheep-man') return true
  // // 返回 falsy 值，不再渲染图标及占位符
  // if (option.key === 'food') return null
  // console.log(agenda.data)
  // console.log(agenda.icon)
  if (agenda.icon) {
    return agenda.icon();
  }
  let icon = agenda.data.icon;
  if (icon === '') {
    icon = '📃';
  }
  return h(NIcon, null, { default: () => h('span', `${icon}`) })
}
</script>
<template>
<!-- <div>
  <n-button @click="to_tasks">task</n-button>
</div> -->
<n-layout-sider
  collapse-mode="width"
  :collapsed-width="64"
  :width="240"
  show-trigger="bar"
  :collapsed="collapsed"
  @collapse="collapsed = true"
  @expand="collapsed = false"
  content-style="height: 100%; position: relative"
  bordered >
  <n-layout position="absolute">
      <n-layout-header style="height: 240px;" bordered>
        <n-space vertical>
          <n-space item-style="width: 100%; display: flex; justify-content: center;">
            <!-- <n-dropdown trigger="click" :options="options">
              <n-button strong secondary style="width: 80%;" type="info"> // TODO List Name</n-button>
            </n-dropdown> -->
            <n-button strong secondary style="width: 80%;" type="info"> // TODO List Name</n-button>
          </n-space>
          <n-space item-style="width: 100%">
            <n-menu
              v-model:value="headMenuKey"
              @update:value="click_side_menu"
              :collapsed="collapsed"
              :collapsed-width="64"
              :collapsed-icon-size="22"
              :options="headMenu"
            />
          </n-space>
        </n-space>
      </n-layout-header>
      <n-layout
        position="absolute"
        style="top: 240px; bottom: 32px"
        :native-scrollbar="false"
        >
          <n-menu
            v-model:value="agendaMenuKey"
            @update:value="click_side_agenda"
            :collapsed="collapsed"
            :collapsed-width="64"
            :collapsed-icon-size="22"
            :render-icon="agenda_icon_render"
            :options="agendaStore.side_agenda_menu"
          />
      </n-layout>
      <n-layout-footer
        bordered
        position="absolute"
        style="height: 32px;"
      >
        footer
      </n-layout-footer>
    </n-layout>
</n-layout-sider>
</template>
