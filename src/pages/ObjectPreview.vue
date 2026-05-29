<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useRouter, useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { inject } from "vue";
const toast = inject<(msg: string, type?: string) => void>("toast", () => {});
import AppIcon from "../components/AppIcon.vue";
import AppSkeleton from "../components/AppSkeleton.vue";

const route = useRoute();
const router = useRouter();
const connectionId = computed(() => route.params.connectionId as string);
const bucket = computed(() => route.params.bucket as string);
const key = computed(() => route.params.key as string);

const loading = ref(true);
const downloading = ref(false);
const errorMsg = ref("");
const previewType = ref<"image" | "video" | "audio" | "text" | "pdf" | "binary">("binary");
const textContent = ref("");
const mediaUrl = ref("");
const fileSize = ref(0);

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

const isText = (ext: string): boolean => {
  const textExts = ["txt", "md", "json", "xml", "yaml", "yml", "toml", "ini",
    "cfg", "conf", "log", "csv", "ts", "js", "tsx", "jsx", "vue", "html", "css",
    "py", "rs", "go", "java", "c", "cpp", "h", "sh", "bash", "env", "bat",
    "sql", "php", "rb", "swift", "kt", "scala", "lua", "r", "makefile", "dockerfile"];
  return textExts.includes(ext) || ext === "";
};

const isPDF = (ext: string): boolean => {
  return ext === "pdf";
};

onMounted(loadPreview);

async function loadPreview() {
  loading.value = true;
  errorMsg.value = "";
  try {
    console.log("[ObjectPreview] 正在获取文件:", {
      connectionId: connectionId.value,
      bucket: bucket.value,
      key: key.value,
    });

    const data = await invoke<number[]>("get_object", {
      connectionId: connectionId.value,
      bucket: bucket.value,
      key: key.value,
    });

    console.log("[ObjectPreview] 收到数据:", data.length, "bytes");

    const bytes = new Uint8Array(data);
    fileSize.value = bytes.length;

    const ext = getFileExt(key.value);
    console.log("[ObjectPreview] 文件扩展名:", ext);

    if (isImage(ext)) {
      console.log("[ObjectPreview] 作为图片预览");
      const blob = new Blob([bytes]);
      mediaUrl.value = URL.createObjectURL(blob);
      previewType.value = "image";
      console.log("[ObjectPreview] mediaUrl:", mediaUrl.value);
    } else if (isVideo(ext)) {
      console.log("[ObjectPreview] 作为视频预览");
      const blob = new Blob([bytes], { type: `video/${ext}` });
      mediaUrl.value = URL.createObjectURL(blob);
      previewType.value = "video";
      console.log("[ObjectPreview] mediaUrl:", mediaUrl.value);
    } else if (isAudio(ext)) {
      console.log("[ObjectPreview] 作为音频预览");
      const blob = new Blob([bytes], { type: `audio/${ext}` });
      mediaUrl.value = URL.createObjectURL(blob);
      previewType.value = "audio";
      console.log("[ObjectPreview] mediaUrl:", mediaUrl.value);
    } else if (isText(ext) || (bytes.length < 512 * 1024)) {
      console.log("[ObjectPreview] 尝试作为文本预览");
      const decoder = new TextDecoder("utf-8", { fatal: false });
      try {
        textContent.value = decoder.decode(bytes);
        previewType.value = "text";
        console.log("[ObjectPreview] 文本预览成功, 长度:", textContent.value.length);
      } catch (e) {
        console.warn("[ObjectPreview] 文本解码失败:", e);
        previewType.value = "binary";
      }
    } else if (isPDF(ext)) {
      console.log("[ObjectPreview] 作为PDF预览");
      const blob = new Blob([bytes], { type: "application/pdf" });
      mediaUrl.value = URL.createObjectURL(blob);
      previewType.value = "pdf";
      console.log("[ObjectPreview] mediaUrl:", mediaUrl.value);
    } else {
      console.log("[ObjectPreview] 作为二进制文件");
      previewType.value = "binary";
    }
  } catch (e: any) {
    console.error("[ObjectPreview] 加载失败:", e);
    errorMsg.value = e?.toString() || "加载失败";
  } finally {
    loading.value = false;
  }
}

async function downloadFile() {
  if (downloading.value) return;
  downloading.value = true;
  try {
    const data = await invoke<number[]>("get_object", {
      connectionId: connectionId.value,
      bucket: bucket.value,
      key: key.value,
    });
    const blob = new Blob([new Uint8Array(data)]);
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = key.value.split("/").pop() || key.value;
    a.click();
    URL.revokeObjectURL(url);
    toast("下载完成", "success");
  } catch (e: any) {
    toast("下载失败: " + (e?.toString() || "未知错误"), "error");
  } finally {
    downloading.value = false;
  }
}

function formatSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB"];
  let i = 0, size = bytes;
  while (size >= 1024 && i < units.length - 1) { size /= 1024; i++; }
  return size.toFixed(i === 0 ? 0 : 1) + " " + units[i];
}

function getFileIcon(): string {
  const ext = getFileExt(key.value);
  if (isImage(ext)) return "image";
  if (isVideo(ext)) return "video-camera";
  if (isAudio(ext)) return "speaker-wave";
  if (isText(ext)) return "document-text";
  if (isPDF(ext)) return "document";
  return "file";
}
</script>

<template>
  <div>
    <div class="preview-header">
      <button class="btn btn-ghost" @click="router.back()">
        <AppIcon name="arrow-left" :size="16" /> 返回
      </button>
      <div class="preview-title">
        <h2 class="file-title">{{ key.split('/').pop() }}</h2>
        <p class="file-path">{{ key }}</p>
      </div>
      <button class="btn btn-ghost" @click="downloadFile" :disabled="downloading">
        <AppIcon v-if="downloading" name="spinner" :size="16" class="spin" />
        <AppIcon v-else name="download" :size="16" />
        {{ downloading ? '下载中...' : '下载' }}
      </button>
    </div>

    <p v-if="errorMsg" class="error-msg" role="alert">{{ errorMsg }}</p>

    <AppSkeleton v-if="loading" type="text" :lines="4" />

    <div v-if="!loading && !errorMsg" class="preview-container">
      <!-- 图片预览 -->
      <div v-if="previewType === 'image'" class="image-preview">
        <img :src="mediaUrl" :alt="key.split('/').pop() || '预览图片'" />
      </div>

      <!-- 视频预览 -->
      <div v-else-if="previewType === 'video'" class="video-preview">
        <video controls :src="mediaUrl" :title="key.split('/').pop() || '预览视频'">
          您的浏览器不支持视频播放
        </video>
      </div>

      <!-- 音频预览 -->
      <div v-else-if="previewType === 'audio'" class="audio-preview">
        <div class="audio-wrapper">
          <AppIcon name="speaker-wave" :size="48" class="audio-icon" />
          <audio controls :src="mediaUrl">
            您的浏览器不支持音频播放
          </audio>
        </div>
      </div>

      <!-- 文本预览 -->
      <div v-else-if="previewType === 'text'" class="text-preview">
        <div class="text-preview-header">
          <span class="text-badge">{{ (textContent?.length || 0).toLocaleString() }} 字符</span>
          <span class="text-badge">{{ formatSize(fileSize) }}</span>
        </div>
        <pre><code>{{ textContent }}</code></pre>
      </div>

      <!-- PDF预览 -->
      <div v-else-if="previewType === 'pdf'" class="pdf-preview">
        <iframe :src="mediaUrl" title="PDF预览" class="pdf-iframe"></iframe>
      </div>

      <!-- 二进制文件 -->
      <div v-else class="binary-preview">
        <AppIcon :name="getFileIcon()" :size="64" />
        <h3>{{ key.split('/').pop() }}</h3>
        <p class="file-info">{{ formatSize(fileSize) }} · 不支持在线预览</p>
        <button class="btn btn-primary" @click="downloadFile" :disabled="downloading">
          <AppIcon v-if="downloading" name="spinner" :size="16" class="spin" />
          <AppIcon v-else name="download" :size="16" />
          {{ downloading ? '下载中...' : '下载查看' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.preview-header {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  margin-bottom: 24px;
}
.preview-title { flex: 1; min-width: 0; }
.file-title {
  font-family: var(--font-heading);
  font-size: 18px;
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.file-path {
  color: var(--color-text-muted);
  font-size: 12px;
  margin-top: 2px;
  font-family: var(--font-heading);
  word-break: break-all;
}
.preview-container {
  background: var(--color-surface);
  border: 1px solid var(--color-border-light);
  border-radius: 12px;
  overflow: hidden;
}

.image-preview {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 24px;
  background: var(--color-bg);
  min-height: 300px;
}
.image-preview img {
  max-width: 100%;
  max-height: 75vh;
  object-fit: contain;
  border-radius: 8px;
}

.video-preview {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 24px;
  background: var(--color-bg);
}
.video-preview video {
  max-width: 100%;
  max-height: 75vh;
  border-radius: 8px;
  width: 100%;
}

.audio-preview {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 48px 24px;
  background: var(--color-bg);
}
.audio-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  padding: 24px;
  background: var(--color-surface);
  border-radius: 12px;
}
.audio-icon {
  opacity: 0.4;
}

.text-preview {
  padding: 0;
}
.text-preview-header {
  display: flex;
  gap: 8px;
  padding: 10px 20px;
  border-bottom: 1px solid var(--color-border-light);
  background: var(--color-bg);
}
.text-badge {
  font-size: 11px;
  color: var(--color-text-muted);
  font-family: var(--font-heading);
  background: var(--color-bg);
  padding: 2px 8px;
  border-radius: 4px;
}
.text-preview pre {
  padding: 20px;
  margin: 0;
  overflow: auto;
  max-height: 70vh;
  font-family: var(--font-heading);
  font-size: 13px;
  line-height: 1.7;
  color: var(--color-text-secondary);
  white-space: pre-wrap;
  word-break: break-word;
  tab-size: 2;
}

.pdf-preview {
  height: 80vh;
}
.pdf-iframe {
  width: 100%;
  height: 100%;
  border: none;
}

.binary-preview {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 24px;
  text-align: center;
  gap: 12px;
}
.binary-preview :deep(svg) { opacity: 0.3; }
.binary-preview h3 {
  font-family: var(--font-heading);
  font-size: 16px;
  font-weight: 500;
  word-break: break-all;
  max-width: 500px;
}
.file-info {
  color: var(--color-text-muted);
  font-size: 13px;
  margin-bottom: 8px;
}
</style>
