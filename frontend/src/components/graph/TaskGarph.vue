<script setup lang="ts">
import { VueFlow, useVueFlow } from '@vue-flow/core';
import { MiniMap } from '@vue-flow/minimap';
import {
  watchEffect,
  // onBeforeUnmount,
  ref,
} from 'vue';
import TaskNode from './TaskNode.vue'
import {
  query_grap_position_by_agenda_id,
  update_or_create_grap_position
} from '../../api/agenda';
import { query_taks_relates_by_agenda_id } from '../../api/task';
import GraphPosition, { Position } from '../../lib/graph_position';
const props = defineProps(['agenda_id', 'tasks']);

const positionHolder = ref<GraphPosition>(new GraphPosition({ belong_agenda_id: props.agenda_id }));
const {
  addNodes,
  addEdges,
  onNodeDragStop,
  // updateNodePositions
} = useVueFlow({})
onNodeDragStop(async ({ node }) => {
  // console.log(node.position)
  // console.log(node.data)
  positionHolder.value.config.set(`${node.data.id}`, new Position({x: node.position.x, y: node.position.y}))
  positionHolder.value = await update_or_create_grap_position(positionHolder.value);
});
// onBeforeUnmount(async () => {
//   console.log('存储内容')
// });

watchEffect(async () => {
  let init_position = await query_grap_position_by_agenda_id(props.agenda_id);
  if (init_position) {
    positionHolder.value = init_position;
  }
  const nodes = [];
  for(const each of props.tasks) {
    let position = positionHolder.value.config.get(`${each.id}`);
    if (position == undefined) {
      position = new Position({x: 0, y: 0})
    }
    nodes.push({
        id: `${each.id}`,
        type: 'custom',
        position: { ...position },
        label: `${each.title}`,
        data: each,
      });
  }
  addNodes(nodes);

  
  if (props.agenda_id !== undefined) {
    const lines = [];
    const realates = await query_taks_relates_by_agenda_id(props.agenda_id);
    console.log(realates);
    for(const edge of realates) {
      lines.push({
        // id: `e${edge.child_id}-${edge.parent_id}`,
        source: `${edge.child_id}`,
        target: `${edge.parent_id}`,
      })
    }
    
    console.log(lines);
    addEdges(lines);
  }
})

</script>
<template>
  <!-- {{ props.agenda_id }}
  {{ props.tasks }} -->
  <div style="height: 100%;">
    <VueFlow >
      <template #node-custom="{ data }">
        <TaskNode :task="data" />
      </template>
      <MiniMap  :pannable="true" :zoomable="true"/>
    </VueFlow>
  </div>
</template>
<style>
/* these are necessary styles for vue flow */
@import '@vue-flow/core/dist/style.css';

/* this contains the default theme, these are optional styles */
@import '@vue-flow/core/dist/theme-default.css';
</style>
