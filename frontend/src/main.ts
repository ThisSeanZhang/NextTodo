import { createApp } from 'vue'

import 'vfonts/FiraCode.css'
import './style.css'

import App from './App.vue'
import store from './store';
import router from './router';
import {
  // create naive ui
  create,
  // component
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
  NLayoutFooter,
  NSpace,
  NInput,
  NInputNumber,
  NInputGroup,
  NInputGroupLabel,
  NCheckbox,
  NButton,
  NSelect,
  NGrid,
  NGi,
  NUpload,
  NThing,
  NIcon,
  NPopconfirm,
  NSpin,
  NDivider,
  NRadioGroup,
  NRadioButton,
  NScrollbar,
  NModal,
  NFormItemGi,
  NDatePicker,
  NForm,
  NDropdown,
  NH2,
  NH6,
  NEllipsis,
  NCard,
  NCollapse,
  NCollapseItem,
  NCollapseTransition,
  NEmpty,
  NMenu,
  NGridItem,
  NTime,
  NTag,
  NDescriptions,
  NDescriptionsItem,
  NSwitch,
} from 'naive-ui'

const naive = create({
components: [
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
  NLayoutFooter,
  NSpace,
  NInput,
  NInputNumber,
  NInputGroup,
  NInputGroupLabel,
  NCheckbox,
  NButton,
  NSelect,
  NGrid,
  NGi,
  NUpload,
  NThing,
  NIcon,
  NPopconfirm,
  NSpin,
  NDivider,
  NRadioGroup,
  NRadioButton,
  NScrollbar,
  NModal,
  NFormItemGi,
  NDatePicker,
  NForm,
  NDropdown,
  NH2,
  NH6,
  NEllipsis,
  NCard,
  NCollapse,
  NCollapseItem,
  NCollapseTransition,
  NEmpty,
  NMenu,
  NDivider,
  NGrid,
  NGridItem,
  NTime,
  NTag,
  NDescriptions,
  NDescriptionsItem,
  NSwitch,
  NIcon,
]
})

createApp(App)
  .use(store)
  .use(naive)
  .use(router)
  .mount('#app')