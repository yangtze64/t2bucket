<script setup lang="ts">
import AppIcon from "./AppIcon.vue";
defineProps<{
  icon: string;
  title: string;
  description?: string;
}>();
</script>

<template>
  <div class="empty-state">
    <AppIcon :name="icon" :size="40" class="empty-icon" />
    <h3>{{ title }}</h3>
    <p v-if="description">{{ description }}</p>
    <slot />
  </div>
</template>

<style scoped>
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 64px 24px;
  text-align: center;
  gap: 12px;
}
.empty-icon { opacity: 0.25; }
h3 { font-size: 16px; font-weight: 500; color: #94a3b8; margin: 0; }
p { font-size: 13px; color: #64748b; margin: 0; }
</style>
