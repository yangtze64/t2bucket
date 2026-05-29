<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import AppIcon from "../components/AppIcon.vue";
import AppSkeleton from "../components/AppSkeleton.vue";
import EmptyState from "../components/EmptyState.vue";
import { inject } from "vue";
const toast = inject<(msg: string, type?: string) => void>("toast", () => {});
const dialog = inject<{ confirm: (msg: string) => Promise<boolean>, confirmWithText: (msg: string, requiredText: string) => Promise<boolean> }>("dialog", { confirm: async (_msg: string) => false, confirmWithText: async (_msg: string, _requiredText: string) => false });
import type { CosConnection } from "../types";
import { providerLabels } from "../types";

const router = useRouter();
const connections = ref<CosConnection[]>([]);
const loading = ref(true);
const errorMsg = ref("");
const testingIds = ref<Set<string>>(new Set());

async function loadConnections() {
  loading.value = true;
  errorMsg.value = "";
  try {
    connections.value = await invoke<CosConnection[]>("list_connections");
  } catch (e: any) {
    errorMsg.value = e?.toString() || "加载失败";
  } finally {
    loading.value = false;
  }
}

async function deleteConnection(id: string, name: string) {
  if (!(await dialog.confirmWithText(`确定删除连接 "${name}"？此操作不可撤销，请输入连接名称来确认。`, name))) return;
  try {
    await invoke("delete_connection", { id });
    toast(`连接 "${name}" 已删除`, "success");
    await loadConnections();
  } catch (e: any) {
    toast("删除失败: " + (e?.toString() || "未知错误"), "error");
  }
}

async function testConnection(conn: CosConnection) {
  testingIds.value = new Set([...testingIds.value, conn.id]);
  try {
    await invoke<string[]>("test_connection", {
      secretId: conn.secret_id,
      secretKey: conn.secret_key,
      region: conn.region,
      provider: conn.provider,
    });
    toast(`"${conn.name}" 连接正常`, "success");
  } catch (e: any) {
    toast(`"${conn.name}" 连接失败: ` + (e?.toString() || "未知错误"), "error");
  } finally {
    const next = new Set(testingIds.value);
    next.delete(conn.id);
    testingIds.value = next;
  }
}

function browseBuckets(id: string) {
  router.push({ name: "browse-buckets", params: { connectionId: id } });
}

function formatDate(ts: number): string {
  if (!ts) return "";
  const d = new Date(ts * 1000);
  return d.toLocaleDateString("zh-CN", { year: "numeric", month: "2-digit", day: "2-digit" });
}

onMounted(loadConnections);
</script>

<template>
  <div>
    <div class="page-header">
      <div>
        <h2>云存储连接</h2>
        <p class="subtitle">管理你的云存储连接</p>
      </div>
      <button class="btn btn-primary" @click="router.push({ name: 'new-connection' })">
        <AppIcon name="plus" :size="16" />
        新建连接
      </button>
    </div>

    <p v-if="errorMsg" class="error-msg" role="alert">{{ errorMsg }}</p>

    <AppSkeleton v-if="loading" type="list" :lines="3" />

    <EmptyState
      v-else-if="connections.length === 0"
      icon="bucket"
      title="暂无连接"
      description="点击上方按钮添加你的第一个云存储连接"
    >
      <button class="btn btn-primary" style="margin-top:12px" @click="router.push({ name: 'new-connection' })">
        <AppIcon name="plus" :size="16" />
        新建连接
      </button>
    </EmptyState>

    <div v-else class="conn-grid">
      <div
        v-for="conn in connections"
        :key="conn.id"
        class="conn-card"
        tabindex="0"
        role="button"
        :aria-label="`浏览 ${conn.name} 的存储桶`"
        @click="browseBuckets(conn.id)"
        @keydown.enter="browseBuckets(conn.id)"
      >
        <div class="conn-card-top">
          <div class="conn-icon">
            <AppIcon name="bucket" :size="20" />
          </div>
          <div class="conn-card-body">
            <span class="conn-name">{{ conn.name }}</span>
            <span class="conn-meta">{{ providerLabels[conn.provider] || conn.provider }} · {{ conn.region }}</span>
            <span v-if="conn.created_at" class="conn-date">{{ formatDate(conn.created_at) }}</span>
          </div>
        </div>
        <div class="conn-actions" @click.stop>
          <button class="btn btn-ghost btn-icon" title="测试连接"
            :disabled="testingIds.has(conn.id)"
            @click="testConnection(conn)">
            <AppIcon v-if="testingIds.has(conn.id)" name="spinner" :size="15" class="spin" />
            <AppIcon v-else name="refresh" :size="15" />
          </button>
          <button class="btn btn-ghost btn-icon" title="编辑"
            @click="router.push({ name: 'edit-connection', params: { id: conn.id } })">
            <AppIcon name="edit" :size="15" />
          </button>
          <button class="btn btn-danger btn-icon" title="删除" @click="deleteConnection(conn.id, conn.name)">
            <AppIcon name="trash" :size="15" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
  gap: 16px;
}
.page-header h2 {
  font-family: var(--font-heading);
  font-size: 22px;
  font-weight: 600;
  letter-spacing: -0.02em;
}
.subtitle {
  color: var(--color-text-muted);
  font-size: 13px;
  margin-top: 4px;
}
.conn-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.conn-card {
  background: var(--color-surface);
  border: 1px solid var(--color-border-light);
  border-radius: 10px;
  padding: 16px 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  transition: all 0.2s ease;
}
.conn-card:hover, .conn-card:focus-visible {
  background: var(--color-surface-hover);
  border-color: var(--color-border);
  transform: translateX(2px);
  outline: none;
}
.conn-card:focus-visible {
  box-shadow: 0 0 0 2px var(--color-primary);
}
.conn-card-top {
  display: flex;
  align-items: center;
  gap: 14px;
}
.conn-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: rgba(96, 165, 250, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-primary);
  flex-shrink: 0;
}
.conn-card-body {
  display: flex;
  flex-direction: column;
}
.conn-name {
  font-weight: 600;
  font-size: 15px;
}
.conn-meta {
  color: var(--color-text-muted);
  font-size: 12px;
  margin-top: 2px;
}
.conn-date {
  color: var(--color-text-muted);
  font-size: 11px;
  margin-top: 1px;
  font-family: var(--font-heading);
}
.conn-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.2s;
}
.conn-card:hover .conn-actions,
.conn-card:focus-visible .conn-actions {
  opacity: 1;
}

@keyframes spin { to { transform: rotate(360deg); } }
.spin { animation: spin 1s linear infinite; }

@media (prefers-reduced-motion: reduce) {
  .conn-card { transition: none; }
  .conn-card:hover { transform: none; }
  .conn-actions { transition: none; }
  .spin { animation: none; }
}
</style>
