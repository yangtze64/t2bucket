<script setup lang="ts">
defineProps<{
  lines?: number;
  type?: "list" | "card" | "text";
}>();
</script>

<template>
  <div v-if="type === 'list'" class="skeleton-list">
    <div v-for="i in (lines || 4)" :key="i" class="skeleton-row">
      <div class="skeleton-icon animate-pulse" />
      <div class="skeleton-line animate-pulse" />
    </div>
  </div>
  <div v-else-if="type === 'card'" class="skeleton-cards">
    <div v-for="i in (lines || 3)" :key="i" class="skeleton-card animate-pulse" />
  </div>
  <div v-else class="skeleton-text">
    <div v-for="i in (lines || 3)" :key="i" class="skeleton-line animate-pulse" :style="{ width: (90 - i * 10) + '%' }" />
  </div>
</template>

<style scoped>
.skeleton-list { display: flex; flex-direction: column; gap: 8px; }
.skeleton-row {
  display: flex; align-items: center; gap: 12px;
  padding: 12px 16px;
  background: var(--color-surface);
  border-radius: 8px;
}
.skeleton-icon { width: 20px; height: 20px; border-radius: 4px; background: var(--color-bg); }
.skeleton-line { height: 14px; border-radius: 4px; background: var(--color-bg); flex: 1; }
.skeleton-cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 12px; }
.skeleton-card { height: 80px; border-radius: 8px; background: var(--color-surface); }
.skeleton-text { display: flex; flex-direction: column; gap: 10px; padding: 20px; }
.animate-pulse { animation: pulse 1.8s ease-in-out infinite; }
@keyframes pulse {
  0%, 100% { opacity: 0.3; }
  50% { opacity: 0.6; }
}
</style>
