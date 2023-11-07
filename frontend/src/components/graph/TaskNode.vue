<script setup lang="ts">
import {
  computed
} from 'vue';
import { Handle, Position } from '@vue-flow/core'
import CardTask  from '../task/CardTask.vue';

const props = defineProps(['task']);

const task = computed(() => props.task);
</script>
<template>
  <n-space>

    <CardTask :task="task" />

  <Handle type="target" :position="Position.Top" />
  <Handle type="source" :position="Position.Bottom" />

  </n-space>
</template>
