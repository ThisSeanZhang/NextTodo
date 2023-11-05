import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
import TaskListVue from '../views/TaskList.vue';
import AgendaView from '../views/AgendaView.vue';
import TaskDetail from '../views/TaskDetail.vue';
const routes: Array<RouteRecordRaw> = [
  {
    path: '/agendas',
    component: AgendaView,
  },
  {
    path: '/agendas/:agenda_id(\\d+)/:type',
    component: TaskListVue
  },
  {
    path: '/agendas/:agenda_id/tasks/:task_id',
    component: TaskDetail
  },
];



const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
