<script setup lang="ts">
import { ref } from "vue";
import AppIcon from "./AppIcon.vue";

interface Toast {
  id: number;
  message: string;
  type: "error" | "success" | "info";
}

const toasts = ref<Toast[]>([]);
let nextId = 0;

function show(message: string, type: Toast["type"] = "error") {
  const id = nextId++;
  toasts.value.push({ id, message, type });
  setTimeout(() => {
    toasts.value = toasts.value.filter((t) => t.id !== id);
  }, 4000);
}

function remove(id: number) {
  toasts.value = toasts.value.filter((t) => t.id !== id);
}

defineExpose({ show });
</script>

<template>
  <Teleport to="body">
    <div class="toast-container">
      <TransitionGroup name="toast">
        <div
          v-for="t in toasts"
          :key="t.id"
          class="toast"
          :class="t.type"
          @click="remove(t.id)"
        >
          <AppIcon
            :name="t.type === 'success' ? 'check' : t.type === 'error' ? 'xmark' : 'info'"
            :size="16"
          />
          <span>{{ t.message }}</span>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<style scoped>
.toast-container {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 9999;
  display: flex;
  flex-direction: column-reverse;
  gap: 8px;
  max-width: 380px;
}
.toast {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 18px;
  border-radius: 10px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  backdrop-filter: blur(12px);
  box-shadow: 0 4px 20px rgba(0,0,0,0.25);
  transition: all 0.2s;
}
.toast:hover {
  transform: translateX(-2px);
}
.toast.error {
  background: rgba(239, 68, 68, 0.92);
  color: #fff;
}
.toast.success {
  background: rgba(34, 197, 94, 0.92);
  color: #0F172A;
}
[data-theme="dark"] .toast.success {
  color: #0F172A;
}
.toast.info {
  background: rgba(96, 165, 250, 0.92);
  color: #fff;
}
/* Transitions */
.toast-enter-active { transition: all 0.3s ease; }
.toast-leave-active { transition: all 0.2s ease; }
.toast-enter-from { opacity: 0; transform: translateX(40px); }
.toast-leave-to { opacity: 0; transform: translateX(40px); }
</style>
