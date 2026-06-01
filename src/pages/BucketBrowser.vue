<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, computed } from "vue";
import { useRouter, useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import AppIcon from "../components/AppIcon.vue";
import AppSkeleton from "../components/AppSkeleton.vue";
import EmptyState from "../components/EmptyState.vue";
import { inject } from "vue";
const toast = inject<(msg: string, type?: string) => void>("toast", () => {});
const dialog = inject<{ confirm: (msg: string) => Promise<boolean>; prompt: (msg: string, def?: string) => Promise<string | null>; confirmWithText: (msg: string, requiredText: string) => Promise<boolean> }>("dialog", { confirm: async (_msg: string) => false, prompt: async (_msg: string, _def?: string) => null, confirmWithText: async (_msg: string, _requiredText: string) => false });
import type { CosConnection, ObjectItem } from "../types";

const router = useRouter();
const route = useRoute();
const connectionId = computed(() => route.params.connectionId as string);
const bucket = computed(() => route.params.bucket as string | undefined);
const prefix = ref("");

const connection = ref<CosConnection | null>(null);
const buckets = ref<string[]>([]);
const objects = ref<ObjectItem[]>([]);
const loading = ref(true);
const errorMsg = ref("");
const uploadInput = ref<HTMLInputElement | null>(null);
const uploading = ref(false);

const pathParts = computed(() =>
  prefix.value ? prefix.value.split("/").filter(Boolean) : []
);

onMounted(loadData);
watch(() => route.fullPath, loadData);

async function loadData() {
  loading.value = true;
  errorMsg.value = "";
  prefix.value = (route.query.prefix as string) || "";
  try {
    const conns = await invoke<CosConnection[]>("list_connections");
    connection.value = conns.find((c) => c.id === connectionId.value) || null;
    if (!connection.value) { errorMsg.value = "连接不存在"; return; }
    if (bucket.value) await loadObjects();
    else await loadBuckets();
  } catch (e: any) {
    errorMsg.value = e?.toString() || "加载失败";
  } finally {
    loading.value = false;
  }
}

async function loadBuckets() {
  try {
  buckets.value = await invoke<string[]>("list_buckets", { connectionId: connectionId.value });
  } catch (e: any) {
    errorMsg.value = "获取桶列表失败: " + (e?.toString() || "网络超时，请检查密钥和网络");
  }
}

async function loadObjects() {
  const result = await invoke<{ items: ObjectItem[]; prefixes: string[] }>(
    "list_objects", {
      connectionId: connectionId.value,
      bucket: bucket.value,
      prefix: prefix.value,
      delimiter: "/",
    }
  );
  const dirs: ObjectItem[] = result.prefixes.map((p: string) => ({
    key: p, size: 0, lastModified: "", isDir: true,
  }));
  objects.value = [...dirs, ...result.items];
}

function goBack() {
  if (bucket.value) {
    if (prefix.value) {
      goUp();
    } else {
      router.push({ name: "browse-buckets", params: { connectionId: connectionId.value } });
    }
  } else {
    router.push({ name: "home" });
  }
}

function enterBucket(name: string) {
  router.push({ name: "browse-objects", params: { connectionId: connectionId.value, bucket: name } });
}

function enterFolder(name: string) {
  router.push({ name: "browse-objects", params: { connectionId: connectionId.value, bucket: bucket.value }, query: { prefix: name } });
}

function goToPath(index: number) {
  const parts = pathParts.value.slice(0, index + 1);
  router.push({ name: "browse-objects", params: { connectionId: connectionId.value, bucket: bucket.value }, query: parts.length > 0 ? { prefix: parts.join("/") + "/" } : {} });
}

function goUp() {
  const parts = pathParts.value.slice(0, -1);
  router.push({ name: "browse-objects", params: { connectionId: connectionId.value, bucket: bucket.value }, query: parts.length > 0 ? { prefix: parts.join("/") + "/" } : {} });
}

async function deleteObject(key: string, isDir: boolean) {
  const label = isDir ? '文件夹' : '文件';
  const displayName = isDir ? key.replace(/\/$/, "").split("/").pop() || key : key.split("/").pop() || key;
  if (!(await dialog.confirmWithText(`确定删除${label} "${displayName}"？此操作不可撤销，请输入文件/文件夹名称来确认。`, displayName))) return;
  try {
    await invoke("delete_object", { connectionId: connectionId.value, bucket: bucket.value, key });
    toast(`${label}已删除`, "success");
    await loadObjects();
  } catch (e: any) {
    toast("删除失败: " + (e?.toString() || "未知错误"), "error");
  }
}

async function downloadObject(key: string) {
  if (downloadingKeys.value.has(key)) return;
  const currentSet = new Set(downloadingKeys.value);
  currentSet.add(key);
  downloadingKeys.value = currentSet;
  
  try {
    const data = await invoke<number[]>("get_object", { connectionId: connectionId.value, bucket: bucket.value, key });
    const blob = new Blob([new Uint8Array(data)]);
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = key.split("/").pop() || key;
    a.click();
    URL.revokeObjectURL(url);
    toast("下载完成", "success");
  } catch (e: any) {
    toast("下载失败: " + (e?.toString() || "未知错误"), "error");
  } finally {
    const newSet = new Set(downloadingKeys.value);
    newSet.delete(key);
    downloadingKeys.value = newSet;
  }
}

function previewObject(key: string) {
  router.push({ name: "preview", params: { connectionId: connectionId.value, bucket: bucket.value, key } });
}

function formatSize(bytes: number): string {
  if (bytes === 0) return "—";
  const units = ["B", "KB", "MB", "GB", "TB"];
  let i = 0, size = bytes;
  while (size >= 1024 && i < units.length - 1) { size /= 1024; i++; }
  return size.toFixed(i === 0 ? 0 : 1) + " " + units[i];
}

function formatDate(d: string): string {
  if (!d) return "—";
  return d.slice(0, 10).replace(/-/g, "/");
}

function triggerUpload() { uploadInput.value?.click(); }

async function handleUpload(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0];
  if (!file) return;
  uploading.value = true;
  try {
    const buf = await file.arrayBuffer();
    const bytes = Array.from(new Uint8Array(buf));
    const key = (prefix.value || "") + file.name;
    await invoke("put_object", { connectionId: connectionId.value, bucket: bucket.value, key, content: bytes });
    toast(`"${file.name}" 上传成功`, "success");
    await loadObjects();
  } catch (e: any) {
    toast("上传失败: " + (e?.toString() || "未知错误"), "error");
  } finally {
    uploading.value = false;
    if (uploadInput.value) uploadInput.value.value = "";
  }
}

async function createFolder() {
  const name = await dialog.prompt("输入文件夹名称：");
  if (!name?.trim()) return;
  const key = (prefix.value || "") + name.trim() + "/";
  try {
    await invoke("put_object", { connectionId: connectionId.value, bucket: bucket.value, key, content: [] });
    toast(`文件夹 "${name.trim()}" 已创建`, "success");
    await loadObjects();
  } catch (e: any) {
    toast("创建失败: " + (e?.toString() || "未知错误"), "error");
  }
}

const downloadingKeys = ref<Set<string>>(new Set());

interface ContextMenuItem {
  label: string;
  icon: string;
  action: () => void;
  danger?: boolean;
}

const ctxMenu = ref(false);
const ctxMenuX = ref(0);
const ctxMenuY = ref(0);
const ctxMenuItems = ref<ContextMenuItem[]>([]);

function showContextMenu(e: MouseEvent, items: ContextMenuItem[]) {
  e.preventDefault();
  e.stopPropagation();
  ctxMenuItems.value = items;
  ctxMenuX.value = e.clientX;
  ctxMenuY.value = e.clientY;
  ctxMenu.value = true;
}

function closeContextMenu() {
  ctxMenu.value = false;
}

function onDocClick() { closeContextMenu(); }
function onDocKeydown(e: KeyboardEvent) { if (e.key === "Escape") closeContextMenu(); }

onMounted(() => {
  document.addEventListener("click", onDocClick);
  document.addEventListener("keydown", onDocKeydown);
});
onBeforeUnmount(() => {
  document.removeEventListener("click", onDocClick);
  document.removeEventListener("keydown", onDocKeydown);
});

async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    toast("已复制到剪贴板", "success");
  } catch {
    toast("复制失败", "error");
  }
}

function onBucketCtx(e: MouseEvent, b: string) {
  showContextMenu(e, [
    { label: "浏览存储桶", icon: "bucket", action: () => enterBucket(b) },
    { label: "复制桶名称", icon: "copy", action: () => copyToClipboard(b) },
  ]);
}

function onFileCtx(e: MouseEvent, obj: ObjectItem) {
  const items: ContextMenuItem[] = [];
  if (!obj.isDir) {
    items.push(
      { label: "预览", icon: "eye", action: () => previewObject(obj.key) },
      { label: "下载", icon: "download", action: () => downloadObject(obj.key) },
    );
  } else {
    items.push({ label: "进入文件夹", icon: "folder", action: () => enterFolder(obj.key) });
  }
  items.push(
    { label: "复制路径", icon: "copy", action: () => copyToClipboard(obj.key) },
    { label: "删除", icon: "trash", action: () => deleteObject(obj.key, obj.isDir), danger: true },
  );
  showContextMenu(e, items);
}

function onEmptyCtx(e: MouseEvent) {
  if (!bucket.value) return;
  showContextMenu(e, [
    { label: "上传文件", icon: "upload", action: triggerUpload },
    { label: "新建文件夹", icon: "folder", action: createFolder },
    { label: "刷新", icon: "refresh", action: () => bucket.value ? loadObjects() : loadBuckets() },
  ]);
}

function getDisplayName(obj: ObjectItem): string {
  if (obj.isDir) {
    const parts = obj.key.replace(/\/$/, "").split("/");
    return parts[parts.length - 1] || obj.key;
  }
  return obj.key.split("/").pop() || obj.key;
}

const getFileExt = (fileName: string): string => {
  const lower = fileName.toLowerCase();
  const lastDot = lower.lastIndexOf(".");
  if (lastDot === -1) return "";
  return lower.slice(lastDot + 1);
};

const isImage = (ext: string): boolean => {
  const imageExts = ["png", "jpg", "jpeg", "gif", "webp", "svg", "bmp", "ico", "tiff", "tif", "avif"];
  return imageExts.includes(ext);
};

const isVideo = (ext: string): boolean => {
  const videoExts = ["mp4", "webm", "ogg", "mov", "avi", "mkv", "flv", "wmv"];
  return videoExts.includes(ext);
};

const isAudio = (ext: string): boolean => {
  const audioExts = ["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma"];
  return audioExts.includes(ext);
};

const isCode = (ext: string): boolean => {
  const codeExts = ["ts", "js", "tsx", "jsx", "vue", "html", "css", "scss", "sass", "less", "py", "rs", "go", "java", "c", "cpp", "h", "sh", "bash", "bat", "sql", "php", "rb", "swift", "kt", "scala", "lua", "r"];
  return codeExts.includes(ext);
};

const isDoc = (ext: string): boolean => {
  const docExts = ["doc", "docx", "txt", "md", "rtf", "pages"];
  return docExts.includes(ext);
};

const isSheet = (ext: string): boolean => {
  const sheetExts = ["xls", "xlsx", "csv", "numbers"];
  return sheetExts.includes(ext);
};

const isPDF = (ext: string): boolean => ext === "pdf";

function getFileIcon(obj: ObjectItem): string {
  if (obj.isDir) return "folder";
  const ext = getFileExt(obj.key);
  if (isImage(ext)) return "image";
  if (isVideo(ext)) return "video-camera";
  if (isAudio(ext)) return "speaker-wave";
  if (isCode(ext)) return "code";
  if (isDoc(ext)) return "document-text";
  if (isSheet(ext)) return "document";
  if (isPDF(ext)) return "document";
  return "file";
}
</script>

<template>
  <div @contextmenu="onEmptyCtx">
    <div class="page-header">
      <button class="btn btn-ghost" @click="goBack">
        <AppIcon name="arrow-left" :size="16" /> 返回
      </button>
      <h2 v-if="connection" class="page-title">{{ connection.name }}</h2>
    </div>

    <p v-if="errorMsg" class="error-msg" role="alert">{{ errorMsg }}</p>

    <template v-if="!loading && !bucket">
      <div class="section-bar">
        <span class="section-label">存储桶 <span class="count">{{ buckets.length }}</span></span>
        <button class="btn btn-ghost btn-sm" @click="loadData()">
          <AppIcon name="refresh" :size="14" /> 刷新
        </button>
      </div>

      <EmptyState v-if="buckets.length === 0" icon="bucket" title="暂无存储桶" description="当前账户下没有可用的 COS 存储桶" />

      <div v-else class="grid-list">
        <div v-for="b in buckets" :key="b" class="grid-card" @click="enterBucket(b)" @contextmenu="onBucketCtx($event, b)">
          <AppIcon name="bucket" :size="22" />
          <span class="grid-name">{{ b }}</span>
          <AppIcon name="chevron-right" :size="16" class="grid-arrow" />
        </div>
      </div>
    </template>

    <template v-if="!loading && bucket">
      <nav class="breadcrumb" aria-label="文件路径导航">
        <button class="crumb" @click="goToPath(-1)">{{ bucket }}</button>
        <template v-for="(part, idx) in pathParts" :key="idx">
          <span class="crumb-sep" aria-hidden="true">/</span>
          <button class="crumb" @click="goToPath(idx)">{{ part }}</button>
        </template>
      </nav>

      <div class="toolbar">
        <div class="toolbar-left">
          <button class="btn btn-primary btn-sm" :disabled="uploading" @click="triggerUpload">
            <AppIcon v-if="uploading" name="spinner" :size="14" class="spin" />
            <AppIcon v-else name="upload" :size="14" />
            {{ uploading ? '上传中...' : '上传文件' }}
          </button>
          <button class="btn btn-ghost btn-sm" @click="createFolder">
            <AppIcon name="folder" :size="14" /> 新建文件夹
          </button>
          <button class="btn btn-ghost btn-sm" @click="loadObjects()">
            <AppIcon name="refresh" :size="14" />
          </button>
        </div>
        <input ref="uploadInput" type="file" style="display:none" @change="handleUpload" />
      </div>

      <div class="data-table" role="table" aria-label="文件列表">
        <div v-if="prefix" class="table-row folder" @click="goUp" role="row">
          <div class="cell-name" role="cell">
            <AppIcon name="folder-open" :size="18" class="file-icon" />
            <span>..</span>
          </div>
          <div class="cell-size" role="cell"></div>
          <div class="cell-date" role="cell"></div>
          <div class="cell-actions" role="cell"></div>
        </div>

        <div
          v-for="obj in objects"
          :key="obj.key"
          class="table-row"
          :class="{ folder: obj.isDir }"
          :role="obj.isDir ? 'button' : 'row'"
          :tabindex="obj.isDir ? 0 : undefined"
          @click="obj.isDir ? enterFolder(obj.key) : undefined"
          @keydown.enter="obj.isDir ? enterFolder(obj.key) : undefined"
          @contextmenu="onFileCtx($event, obj)"
        >
          <div class="cell-name" role="cell">
            <AppIcon :name="getFileIcon(obj)" :size="18" class="file-icon" />
            <span class="item-name" :title="obj.key">{{ getDisplayName(obj) }}</span>
          </div>
          <div class="cell-size" :class="{ muted: obj.isDir }" role="cell">
            {{ obj.isDir ? '—' : formatSize(obj.size) }}
          </div>
          <div class="cell-date muted" role="cell">
            {{ formatDate(obj.lastModified) }}
          </div>
          <div class="cell-actions" v-if="!obj.isDir" role="cell">
            <button class="btn btn-ghost btn-icon btn-sm" title="预览" @click.stop="previewObject(obj.key)">
              <AppIcon name="eye" :size="15" />
            </button>
            <button class="btn btn-ghost btn-icon btn-sm" title="下载" :disabled="downloadingKeys.has(obj.key)" @click.stop="downloadObject(obj.key)">
              <AppIcon v-if="downloadingKeys.has(obj.key)" name="spinner" :size="15" class="spin" />
              <AppIcon v-else name="download" :size="15" />
            </button>
            <button class="btn btn-ghost btn-icon btn-sm btn-danger-ghost" title="删除" @click.stop="deleteObject(obj.key, false)">
              <AppIcon name="trash" :size="15" />
            </button>
          </div>
          <div class="cell-actions" v-else role="cell">
            <button class="btn btn-ghost btn-icon btn-sm btn-danger-ghost" title="删除" @click.stop="deleteObject(obj.key, true)">
              <AppIcon name="trash" :size="15" />
            </button>
          </div>
        </div>
      </div>

      <EmptyState
        v-if="objects.length === 0 && !prefix"
        icon="folder-open"
        title="空存储桶"
        description="上传文件或创建文件夹开始使用"
      />
      <EmptyState
        v-if="objects.length === 0 && prefix"
        icon="folder"
        title="空文件夹"
        description="此文件夹中暂无内容"
      />
    </template>

    <AppSkeleton v-if="loading" type="list" :lines="5" />

    <Teleport to="body">
      <div v-if="ctxMenu" class="ctx-overlay" @click="closeContextMenu" @contextmenu.prevent="closeContextMenu">
        <div class="ctx-menu" :style="{ left: ctxMenuX + 'px', top: ctxMenuY + 'px' }" @click.stop>
          <button
            v-for="(item, idx) in ctxMenuItems"
            :key="idx"
            class="ctx-item"
            :class="{ 'ctx-danger': item.danger }"
            @click="item.action(); closeContextMenu()"
          >
            <AppIcon :name="item.icon" :size="15" />
            <span>{{ item.label }}</span>
          </button>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.page-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
}
.page-title {
  font-family: var(--font-heading);
  font-size: 20px;
  font-weight: 600;
}

.section-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}
.section-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-secondary);
}
.count {
  color: var(--color-text-muted);
  font-weight: 400;
  margin-left: 4px;
}

.grid-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 10px;
}
.grid-card {
  background: var(--color-surface);
  border: 1px solid var(--color-border-light);
  border-radius: 10px;
  padding: 18px 20px;
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
}
.grid-card:hover {
  background: var(--color-surface-hover);
  border-color: var(--color-border);
  transform: translateY(-1px);
}
.grid-name {
  flex: 1;
  font-weight: 500;
  font-size: 14px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.grid-arrow {
  opacity: 0;
  transition: opacity 0.2s, transform 0.2s;
  color: var(--color-text-muted);
}
.grid-card:hover .grid-arrow {
  opacity: 1;
  transform: translateX(2px);
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 2px;
  margin-bottom: 16px;
  font-size: 13px;
  flex-wrap: wrap;
}
.crumb {
  background: none;
  border: none;
  color: var(--color-primary);
  cursor: pointer;
  font-size: 13px;
  font-family: var(--font-body);
  padding: 2px 4px;
  border-radius: 4px;
  transition: background 0.15s;
}
.crumb:hover { background: rgba(96, 165, 250, 0.1); }
.crumb-sep { color: var(--color-text-muted); font-family: var(--font-heading); }

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}
.toolbar-left { display: flex; gap: 6px; flex-wrap: wrap; }

.data-table {
  background: var(--color-surface);
  border: 1px solid var(--color-border-light);
  border-radius: 10px;
  overflow: hidden;
}

.table-row {
  display: grid;
  grid-template-columns: 1fr 100px 120px 120px;
  align-items: center;
  padding: 11px 18px;
  border-bottom: 1px solid var(--color-border-light);
  transition: background 0.15s;
  cursor: default;
}
.table-row:last-child { border-bottom: none; }
.table-row.folder { cursor: pointer; }
.table-row:hover { background: var(--color-surface-hover); }

.cell-name {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}
.file-icon { color: var(--color-text-muted); flex-shrink: 0; }
.folder .file-icon { color: var(--color-primary); }
.item-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
}
.cell-size {
  font-size: 12px;
  font-family: var(--font-heading);
  text-align: right;
  padding-right: 10px;
}
.cell-date { font-size: 12px; text-align: right; padding-right: 4px; }
.muted { color: var(--color-text-muted); }
.cell-actions {
  display: flex;
  gap: 2px;
  justify-content: flex-end;
  opacity: 0;
  transition: opacity 0.15s;
}
.table-row:hover .cell-actions { opacity: 1; }
.no-action:hover .cell-actions { opacity: 1; }

.btn-danger-ghost:hover {
  color: var(--color-danger) !important;
  background: rgba(239, 68, 68, 0.12) !important;
}

@keyframes spin { to { transform: rotate(360deg); } }
.spin { animation: spin 1s linear infinite; }

@media (prefers-reduced-motion: reduce) {
  .spin { animation: none; }
  .grid-card:hover { transform: none; }
  .grid-arrow { transition: none; }
  .table-row { transition: none; }
  .cell-actions { transition: none; }
}
</style>

<style>
.ctx-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
}
.ctx-menu {
  position: fixed;
  min-width: 180px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 10px;
  padding: 4px;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.3);
  z-index: 10000;
}
.ctx-item {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  background: none;
  color: var(--color-text-secondary);
  font-size: 13px;
  font-family: var(--font-body);
  cursor: pointer;
  border-radius: 6px;
  transition: background 0.12s, color 0.12s;
}
.ctx-item:hover {
  background: var(--color-surface-hover);
  color: var(--color-text);
}
.ctx-item.ctx-danger:hover {
  background: rgba(239, 68, 68, 0.12);
  color: var(--color-danger);
}
</style>
