<template>
  <div v-if="show" class="context-menu" :style="{ top: `${y}px`, left: `${x}px` }">
    <ul>
      <li v-for="item in menuItems" :key="item.label" @click="handleItemClick(item)">
        {{ item.label }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  x: number;
  y: number;
  show: boolean;
  menuItems: { label: string; action: () => void }[];
}>();

const emit = defineEmits(['close']);

function handleItemClick(item: { label: string; action: () => void }) {
  item.action();
  emit('close');
}
</script>

<style lang="scss" scoped>
@use "~/assets/styles/variables" as v;

.context-menu {
  position: fixed;
  background: v.$background;
  border: 1px solid v.$accent;
  z-index: 1000;
}

ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}

li {
  padding: 10px 16px;
  cursor: pointer;
}

li:hover {
  background-color: v.$element;
}
</style>