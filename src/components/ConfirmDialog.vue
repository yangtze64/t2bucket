<script setup lang="ts">
import { ref, computed } from "vue";

const visible = ref(false);
const message = ref("");
const inputLabel = ref("");
const inputValue = ref("");
const showInput = ref(false);
const requiredConfirmText = ref("");
let resolvePromise: ((value: string | false) => void) | null = null;

function confirm(msg: string): Promise<boolean> {
  return new Promise((resolve) => {
    message.value = msg;
    showInput.value = false;
    requiredConfirmText.value = "";
    inputValue.value = "";
    visible.value = true;
    resolvePromise = (v) => resolve(v !== false);
  });
}

function prompt(msg: string, defaultValue = ""): Promise<string | null> {
  return new Promise((resolve) => {
    message.value = msg;
    inputLabel.value = msg;
    inputValue.value = defaultValue;
    requiredConfirmText.value = "";
    showInput.value = true;
    visible.value = true;
    resolvePromise = (v) => resolve(v === false ? null : v);
  });
}

function confirmWithText(msg: string, requiredText: string): Promise<boolean> {
  return new Promise((resolve) => {
    message.value = msg;
    inputLabel.value = "请输入以下文字来确认";
    inputValue.value = "";
    requiredConfirmText.value = requiredText;
    showInput.value = true;
    visible.value = true;
    resolvePromise = (v) => resolve(v !== false);
  });
}

const canConfirm = computed(() => {
  if (requiredConfirmText.value) {
    return inputValue.value === requiredConfirmText.value;
  }
  return true;
});

function onOk() {
  if (!canConfirm.value) return;
  visible.value = false;
  resolvePromise?.(showInput.value ? inputValue.value : "");
}

function onCancel() {
  visible.value = false;
  resolvePromise?.(false);
}

defineExpose({ confirm, prompt, confirmWithText });
</script>

<template>
  <Teleport to="body">
    <div v-if="visible" class="overlay" @click.self="onCancel">
      <div class="dialog">
        <p class="dialog-msg">{{ message }}</p>
        <div v-if="showInput" class="input-container">
          <p v-if="requiredConfirmText" class="confirm-text-label">请输入 "{{ requiredConfirmText }}" 来确认：</p>
          <input
            v-model="inputValue"
            class="dialog-input"
            :class="{ 'invalid': !canConfirm && requiredConfirmText }"
            @keyup.enter="onOk"
            autofocus
          />
        </div>
        <div class="dialog-actions">
          <button class="btn btn-ghost" @click="onCancel">取消</button>
          <button class="btn btn-primary" @click="onOk" :disabled="!canConfirm">确定</button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
}
.dialog {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 24px;
  min-width: 340px;
  max-width: 440px;
  box-shadow: 0 8px 32px rgba(0,0,0,0.3);
}
.dialog-msg {
  font-size: 14px;
  margin-bottom: 16px;
  color: var(--color-text);
  line-height: 1.6;
}
.input-container {
  margin-bottom: 16px;
}
.confirm-text-label {
  font-size: 13px;
  color: var(--color-text-muted);
  margin-bottom: 8px;
}
.dialog-input {
  width: 100%;
}
.dialog-input.invalid {
  border-color: var(--color-danger);
}
.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
