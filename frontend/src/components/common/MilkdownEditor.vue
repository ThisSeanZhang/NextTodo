<script setup lang="ts">
import { ref, type Ref, computed } from 'vue';
import CodeMirror from 'vue-codemirror6';
import VueMarkdown from 'vue-markdown-wasm';
import { oneDark } from '@codemirror/theme-one-dark';
import { markdown } from '@codemirror/lang-markdown';

const cm: Ref<InstanceType<typeof CodeMirror> | undefined> = ref();
const props = defineProps(['value']);
const emit = defineEmits(['update:value'])
  // const input: Ref<string> = ref(`# The quick brown fox jumps over the lazy dog.`);
// Sync dark mode
// defineProps({ dark: Boolean });

const input = computed({
  get() {
    return props.value
  },
  set(value) {
    emit('update:value', value)
  }
})
</script>

<template>
  <n-grid cols="2" item-responsive responsive="screen" :x-gap="20">
    <n-grid-item span="0 m:1 l:1">
      <n-scrollbar style="max-height: 200px;padding: 14px;" trigger="hover">
        <code-mirror
          :style="{
            height: '200px',
            background: '#282c34'
          }"
          ref="cm"
          v-model="input"
          :lang="markdown()"
          wrap
          basic
          tab
          :extensions=[oneDark]
        />
      </n-scrollbar>
    </n-grid-item>
    <n-grid-item span="0 m:1 l:1">
      
      <n-scrollbar style="max-height: 200px" trigger="hover">
      <vue-markdown v-model="input" />
      </n-scrollbar>
    </n-grid-item>
  </n-grid>
</template>
