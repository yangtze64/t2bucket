<script setup lang="ts">
import { ref, provide, onMounted } from "vue";
import { getVersion } from "@tauri-apps/api/app";
import AppIcon from "./components/AppIcon.vue";
import ConfirmDialog from "./components/ConfirmDialog.vue";
import AppToast from "./components/AppToast.vue";
import { useTheme } from "./composables/useTheme";

const { effectiveTheme, toggleTheme } = useTheme();
const toastRef = ref<InstanceType<typeof AppToast> | null>(null);
const dialogRef = ref<InstanceType<typeof ConfirmDialog> | null>(null);
const appVersion = ref("");
function toast(message: string, type: "error" | "success" | "info" = "error") {
  toastRef.value?.show(message, type);
}

onMounted(async () => {
  try { appVersion.value = await getVersion(); } catch {}
});

provide("toast", toast);
provide("dialog", { confirm: (msg: string) => dialogRef.value?.confirm(msg) ?? Promise.resolve(false), prompt: (msg: string, def?: string) => dialogRef.value?.prompt(msg, def) ?? Promise.resolve(null), confirmWithText: (msg: string, requiredText: string) => dialogRef.value?.confirmWithText(msg, requiredText) ?? Promise.resolve(false) });</script>

<template>
  <div id="app-shell">
    <header class="app-header">
      <router-link to="/" class="app-brand">
        <img src="/app-icon.png" alt="T2Bucket" class="app-logo" />
        <span>T2Bucket</span>
        <span v-if="appVersion" class="app-version">v{{ appVersion }}</span>
      </router-link>
      <div class="header-right">
        <button class="theme-toggle" @click="toggleTheme" :title="effectiveTheme() === 'dark' ? '切换亮色' : '切换暗色'">
          <AppIcon :name="effectiveTheme() === 'dark' ? 'sun' : 'moon'" :size="15" />
        </button>
      </div>
    </header>
    <main class="app-main">
      <router-view v-slot="{ Component }">
        <transition name="page-fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
    <AppToast ref="toastRef" />
    <ConfirmDialog ref="dialogRef" />
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Fira+Code:wght@400;500;600&family=Fira+Sans:wght@300;400;500;600;700&display=swap');

:root {
  --color-bg: #0F172A;
  --color-surface: #1E293B;
  --color-surface-hover: #273549;
  --color-border: #334155;
  --color-border-light: #1E293B;
  --color-primary: #60A5FA;
  --color-cta: #22C55E;
  --color-cta-hover: #16A34A;
  --color-danger: #EF4444;
  --color-danger-hover: #DC2626;
  --color-text: #F8FAFC;
  --color-text-secondary: #94A3B8;
  --color-text-muted: #64748B;
  --font-heading: 'Fira Code', monospace;
  --font-body: 'Fira Sans', system-ui, -apple-system, sans-serif;
}

[data-theme="light"] {
  --color-bg: #F1F5F9;
  --color-surface: #FFFFFF;
  --color-surface-hover: #F1F5F9;
  --color-border: #CBD5E1;
  --color-border-light: #E2E8F0;
  --color-primary: #2563EB;
  --color-cta: #22C55E;
  --color-cta-hover: #16A34A;
  --color-danger: #EF4444;
  --color-danger-hover: #DC2626;
  --color-text: #0F172A;
  --color-text-secondary: #475569;
  --color-text-muted: #94A3B8;
}

* { margin: 0; padding: 0; box-sizing: border-box; }

body {
  font-family: var(--font-body);
  background: var(--color-bg);
  color: var(--color-text);
  font-size: 14px;
  line-height: 1.5;
  -webkit-font-smoothing: antialiased;
}

.app-header {
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  padding: 0 24px;
  height: 52px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-right { display: flex; align-items: center; }

.theme-toggle {
  display: flex; align-items: center; justify-content: center;
  width: 32px; height: 32px; border-radius: 8px;
  border: 1px solid var(--color-border);
  background: transparent; color: var(--color-text-secondary);
  cursor: pointer; transition: all 0.2s;
}
.theme-toggle:hover {
  background: var(--color-surface-hover);
  color: var(--color-text);
  border-color: var(--color-text-muted);
}

.app-brand {
  display: flex; align-items: center; gap: 10px;
  color: var(--color-text); text-decoration: none;
  font-family: var(--font-heading); font-size: 15px; font-weight: 600;
  letter-spacing: -0.02em; transition: color 0.2s;
}
.app-brand:hover { color: var(--color-primary); }
.app-logo { width: 26px; height: 26px; border-radius: 6px; }
.app-version { font-size: 11px; font-weight: 400; color: var(--color-text-muted); letter-spacing: 0; }

.app-main { max-width: 1100px; margin: 0 auto; padding: 32px 20px; }

.page-fade-enter-active, .page-fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.page-fade-enter-from { opacity: 0; transform: translateY(6px); }
.page-fade-leave-to { opacity: 0; transform: translateY(-6px); }

.btn {
  display: inline-flex; align-items: center; gap: 6px;
  border: 1px solid var(--color-border); border-radius: 8px;
  cursor: pointer; padding: 8px 16px; font-size: 13px; font-weight: 500;
  font-family: var(--font-body); background: var(--color-surface);
  color: var(--color-text-secondary); transition: all 0.2s ease; white-space: nowrap;
}
.btn:hover { background: var(--color-surface-hover); color: var(--color-text); border-color: var(--color-text-muted); }
.btn:active { transform: scale(0.98); }
.btn:disabled { opacity: 0.5; cursor: not-allowed; }

.btn-primary { background: var(--color-cta); color: #0F172A; border-color: var(--color-cta); font-weight: 600; }
[data-theme="light"] .btn-primary { color: #FFFFFF; }
.btn-primary:hover { background: var(--color-cta-hover); border-color: var(--color-cta-hover); color: #0F172A; }
[data-theme="light"] .btn-primary:hover { color: #FFFFFF; }

.btn-danger { background: transparent; border-color: transparent; color: var(--color-text-muted); }
.btn-danger:hover { background: rgba(239, 68, 68, 0.12); color: var(--color-danger); border-color: transparent; }

.btn-ghost { background: transparent; border-color: transparent; color: var(--color-text-muted); }
.btn-ghost:hover { background: var(--color-surface-hover); color: var(--color-text); }

.btn-sm { padding: 5px 10px; font-size: 12px; border-radius: 6px; }
.btn-icon { padding: 6px; min-width: auto; }

input, select, textarea {
  padding: 9px 12px; background: var(--color-bg); border: 1px solid var(--color-border);
  border-radius: 8px; font-size: 14px; font-family: var(--font-body);
  color: var(--color-text); transition: border-color 0.2s, box-shadow 0.2s;
}
input:focus, select:focus, textarea:focus {
  outline: none; border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.15);
}
[data-theme="light"] input:focus, [data-theme="light"] select:focus, [data-theme="light"] textarea:focus {
  box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.15);
}
input::placeholder { color: var(--color-text-muted); }

::-webkit-scrollbar { width: 6px; height: 6px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb { background: var(--color-border); border-radius: 3px; }
::-webkit-scrollbar-thumb:hover { background: var(--color-text-muted); }

.btn:focus-visible, .theme-toggle:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}

.btn-primary:focus-visible {
  outline-color: #FFFFFF;
}

@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
  .page-fade-enter-active, .page-fade-leave-active {
    transition: none;
  }
  .btn:active { transform: none; }
}

.error-msg {
  color: #FCA5A5; background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  padding: 10px 14px; border-radius: 8px; margin-bottom: 16px; font-size: 13px;
}
[data-theme="light"] .error-msg {
  color: #991B1B; background: #FEF2F2; border-color: #FECACA;
}
</style>
