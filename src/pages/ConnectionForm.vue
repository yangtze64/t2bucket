<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { useRouter, useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import AppIcon from "../components/AppIcon.vue";
import type { CosConnection } from "../types";
import { inject } from "vue";
const toast = inject<(msg: string, type?: string) => void>("toast", () => {});

const router = useRouter();
const route = useRoute();
const editId = (route.params.id as string) || "";

const name = ref("");
const secretId = ref("");
const secretKey = ref("");
const showSecretKey = ref(false);
const provider = ref("cos");
const region = ref("ap-beijing");
const saving = ref(false);
const errorMsg = ref("");
const touched = ref({ name: false, secretId: false, secretKey: false });

const isFormValid = computed(() => {
  return name.value.trim() && secretId.value.trim() && secretKey.value.trim();
});

const providers = [
  { value: "cos", label: "Tencent COS", available: true },
  { value: "s3", label: "AWS S3", available: true },
  { value: "oss", label: "Aliyun OSS", available: false },
  { value: "obs", label: "Huawei OBS", available: false },
  { value: "minio", label: "MinIO", available: false },
];

const cosRegionGroups = [
  { label: "中国", options: [
    { value: "ap-beijing", label: "ap-beijing" }, { value: "ap-nanjing", label: "ap-nanjing" },
    { value: "ap-shanghai", label: "ap-shanghai" }, { value: "ap-guangzhou", label: "ap-guangzhou" },
    { value: "ap-chengdu", label: "ap-chengdu" }, { value: "ap-chongqing", label: "ap-chongqing" },
    { value: "ap-hongkong", label: "ap-hongkong" },
  ]},
  { label: "亚太", options: [
    { value: "ap-singapore", label: "ap-singapore" }, { value: "ap-bangkok", label: "ap-bangkok" },
    { value: "ap-mumbai", label: "ap-mumbai" }, { value: "ap-seoul", label: "ap-seoul" },
    { value: "ap-tokyo", label: "ap-tokyo" },
  ]},
  { label: "北美", options: [
    { value: "na-siliconvalley", label: "na-siliconvalley" }, { value: "na-ashburn", label: "na-ashburn" },
  ]},
  { label: "欧洲", options: [
    { value: "eu-frankfurt", label: "eu-frankfurt" }, { value: "eu-moscow", label: "eu-moscow" },
  ]},
];

const s3RegionGroups = [
  { label: "中国", options: [
    { value: "cn-north-1", label: "cn-north-1 (北京)" },
    { value: "cn-northwest-1", label: "cn-northwest-1 (宁夏)" },
  ]},
  { label: "美国东部", options: [
    { value: "us-east-1", label: "us-east-1 (N. Virginia)" },
    { value: "us-east-2", label: "us-east-2 (Ohio)" },
  ]},
  { label: "美国西部", options: [
    { value: "us-west-1", label: "us-west-1 (N. California)" },
    { value: "us-west-2", label: "us-west-2 (Oregon)" },
  ]},
  { label: "亚太", options: [
    { value: "ap-east-1", label: "ap-east-1 (Hong Kong)" },
    { value: "ap-south-1", label: "ap-south-1 (Mumbai)" },
    { value: "ap-south-2", label: "ap-south-2 (Hyderabad)" },
    { value: "ap-southeast-1", label: "ap-southeast-1 (Singapore)" },
    { value: "ap-southeast-2", label: "ap-southeast-2 (Sydney)" },
    { value: "ap-southeast-3", label: "ap-southeast-3 (Jakarta)" },
    { value: "ap-southeast-4", label: "ap-southeast-4 (Melbourne)" },
    { value: "ap-northeast-1", label: "ap-northeast-1 (Tokyo)" },
    { value: "ap-northeast-2", label: "ap-northeast-2 (Seoul)" },
    { value: "ap-northeast-3", label: "ap-northeast-3 (Osaka)" },
  ]},
  { label: "加拿大", options: [
    { value: "ca-central-1", label: "ca-central-1 (Central)" },
    { value: "ca-west-1", label: "ca-west-1 (Calgary)" },
  ]},
  { label: "欧洲", options: [
    { value: "eu-central-1", label: "eu-central-1 (Frankfurt)" },
    { value: "eu-central-2", label: "eu-central-2 (Zurich)" },
    { value: "eu-west-1", label: "eu-west-1 (Ireland)" },
    { value: "eu-west-2", label: "eu-west-2 (London)" },
    { value: "eu-west-3", label: "eu-west-3 (Paris)" },
    { value: "eu-south-1", label: "eu-south-1 (Milan)" },
    { value: "eu-south-2", label: "eu-south-2 (Spain)" },
    { value: "eu-north-1", label: "eu-north-1 (Stockholm)" },
  ]},
  { label: "中东", options: [
    { value: "me-south-1", label: "me-south-1 (Bahrain)" },
    { value: "me-central-1", label: "me-central-1 (UAE)" },
  ]},
  { label: "南美洲", options: [
    { value: "sa-east-1", label: "sa-east-1 (São Paulo)" },
  ]},
  { label: "非洲", options: [
    { value: "af-south-1", label: "af-south-1 (Cape Town)" },
  ]},
  { label: "以色列", options: [
    { value: "il-central-1", label: "il-central-1 (Tel Aviv)" },
  ]},
];

const regionGroups = computed(() => {
  return provider.value === "s3" ? s3RegionGroups : cosRegionGroups;
});

watch(provider, (val) => {
  const groups = val === "s3" ? s3RegionGroups : cosRegionGroups;
  region.value = groups[0].options[0].value;
});

onMounted(async () => {
  if (editId) {
    try {
      const conns = await invoke<CosConnection[]>("list_connections");
      const conn = conns.find((c) => c.id === editId);
      if (conn) {
        name.value = conn.name;
        secretId.value = conn.secret_id;
        provider.value = conn.provider;
        region.value = conn.region;
      }
    } catch (e: any) {
      errorMsg.value = "加载连接信息失败";
    }
  }
});

async function save() {
  if (!isFormValid.value) {
    touched.value = { name: true, secretId: true, secretKey: true };
    return;
  }
  saving.value = true;
  errorMsg.value = "";
  try {
    if (editId) {
      await invoke("update_connection", {
        id: editId,
        name: name.value.trim(),
        secretId: secretId.value.trim(),
        secretKey: secretKey.value.trim(),
        provider: provider.value,
        region: region.value,
      });
      toast("连接已更新", "success");
    } else {
      await invoke("add_connection", {
        name: name.value.trim(),
        secretId: secretId.value.trim(),
        secretKey: secretKey.value.trim(),
        region: region.value,
        provider: provider.value,
      });
      toast("连接已创建", "success");
    }
    router.push({ name: "home" });
  } catch (e: any) {
    errorMsg.value = e?.toString() || "保存失败";
  } finally {
    saving.value = false;
  }
}

function fieldError(field: keyof typeof touched.value): string | null {
  const val = { name: name.value, secretId: secretId.value, secretKey: secretKey.value }[field];
  if (touched.value[field] || errorMsg.value) {
    return val.trim() ? null : "必填";
  }
  return null;
}
</script>

<template>
  <div class="form-page">
    <header class="form-header">
      <button class="btn btn-ghost" @click="router.back()">
        <AppIcon name="arrow-left" :size="16" /> 返回
      </button>
      <h2>{{ editId ? '编辑连接' : '新建连接' }}</h2>
    </header>

    <p v-if="errorMsg" class="error-msg" role="alert">{{ errorMsg }}</p>

    <div class="form-card">
      <label class="field">
        <span class="field-label">存储提供商</span>
        <div class="provider-grid">
          <button
            v-for="p in providers" :key="p.value" type="button"
            class="provider-card"
            :class="{ active: provider === p.value, disabled: !p.available }"
            :disabled="!p.available"
            @click="p.available && (provider = p.value)"
          >
            <span class="provider-name">{{ p.label }}</span>
            <span v-if="!p.available" class="coming-soon">即将支持</span>
          </button>
        </div>
      </label>

      <label class="field">
        <span class="field-label">
          连接名称
          <span v-if="fieldError('name')" class="required-mark" role="alert">必填</span>
        </span>
        <input v-model="name" placeholder="例如：我的腾讯云"
          @blur="touched.name = true"
          :class="{ 'input-error': fieldError('name') }" />
      </label>

      <label class="field">
        <span class="field-label">
          {{ provider === 'cos' ? 'SecretId' : 'Access Key' }}
          <span v-if="fieldError('secretId')" class="required-mark" role="alert">必填</span>
        </span>
        <input v-model="secretId"
          :placeholder="provider === 'cos' ? 'AKIDxxxxxxxxxxxxxxxx' : 'AKIAIOSFODNN7EXAMPLE'"
          @blur="touched.secretId = true"
          :class="{ 'input-error': fieldError('secretId') }" />
      </label>

      <label class="field">
        <span class="field-label">
          {{ provider === 'cos' ? 'SecretKey' : 'Secret Key' }}
          <span v-if="fieldError('secretKey')" class="required-mark" role="alert">必填</span>
        </span>
        <div class="password-wrapper">
          <input v-model="secretKey" :type="showSecretKey ? 'text' : 'password'"
            placeholder="••••••••"
            @blur="touched.secretKey = true"
            :class="{ 'input-error': fieldError('secretKey') }" />
          <button type="button" class="password-toggle" @click="showSecretKey = !showSecretKey" :aria-label="showSecretKey ? '隐藏密钥' : '显示密钥'">
            <AppIcon :name="showSecretKey ? 'eyeOff' : 'eye'" :size="16" />
          </button>
        </div>
      </label>

      <label class="field">
        <span class="field-label">Region</span>
        <div class="select-wrapper">
          <select v-model="region">
            <optgroup v-for="g in regionGroups" :key="g.label" :label="g.label">
              <option v-for="r in g.options" :key="r.value" :value="r.value">{{ r.label }}</option>
            </optgroup>
          </select>
          <svg class="select-chevron" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
        </div>
      </label>

      <div class="form-actions">
        <button class="btn btn-ghost" @click="router.back()">取消</button>
        <button class="btn btn-primary" :disabled="!isFormValid || saving" @click="save">
          <AppIcon v-if="saving" name="spinner" :size="16" class="spin" />
          <AppIcon v-else-if="!editId" name="plus" :size="16" />
          {{ saving ? '保存中...' : editId ? '更新连接' : '保存连接' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.form-page { max-width: 520px; margin: 0 auto; }
.form-header { display: flex; align-items: center; gap: 16px; margin-bottom: 24px; }
.form-header h2 { font-family: var(--font-heading); font-size: 20px; font-weight: 600; }
.form-card {
  background: var(--color-surface); border: 1px solid var(--color-border-light);
  border-radius: 12px; padding: 28px; display: flex; flex-direction: column; gap: 20px;
}
.field { display: flex; flex-direction: column; gap: 6px; }
.field-label { font-size: 13px; font-weight: 500; color: var(--color-text-secondary); display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.required-mark {
  font-size: 11px; color: var(--color-danger); font-weight: 600;
  background: rgba(239, 68, 68, 0.1); padding: 1px 7px; border-radius: 4px;
}
.input-error { border-color: var(--color-danger) !important; }
.form-actions { display: flex; justify-content: flex-end; gap: 10px; margin-top: 8px; }

.provider-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(130px, 1fr)); gap: 8px; }
.provider-card {
  display: flex; flex-direction: column; align-items: center; gap: 4px;
  padding: 12px 8px; background: var(--color-bg); border: 1px solid var(--color-border);
  border-radius: 8px; cursor: pointer; transition: all 0.2s;
  font-family: var(--font-body); font-size: 14px; color: var(--color-text-secondary); text-align: center;
}
.provider-card:hover:not(.disabled) { border-color: var(--color-primary); color: var(--color-text); }
.provider-card.active { border-color: var(--color-cta); color: var(--color-cta); background: rgba(34, 197, 94, 0.08); }
.provider-card.disabled { opacity: 0.4; cursor: not-allowed; }
.provider-name { font-weight: 600; }
.coming-soon { font-size: 11px; color: var(--color-text-muted); background: var(--color-bg); padding: 1px 6px; border-radius: 4px; font-family: var(--font-heading); }

.select-wrapper { position: relative; }
.select-wrapper select { width: 100%; padding-right: 36px; appearance: none; -webkit-appearance: none; cursor: pointer; }
.select-chevron { position: absolute; right: 12px; top: 50%; transform: translateY(-50%); pointer-events: none; color: var(--color-text-muted); }

.password-wrapper { position: relative; }
.password-wrapper input { width: 100%; padding-right: 40px; }
.password-toggle { position: absolute; right: 6px; top: 50%; transform: translateY(-50%); background: none; border: none; color: var(--color-text-muted); cursor: pointer; padding: 4px; display: flex; align-items: center; border-radius: 4px; transition: color 0.15s; }
.password-toggle:hover { color: var(--color-text); }

@keyframes spin { to { transform: rotate(360deg); } }
.spin { animation: spin 1s linear infinite; }

@media (prefers-reduced-motion: reduce) {
  .spin { animation: none; }
}
</style>
