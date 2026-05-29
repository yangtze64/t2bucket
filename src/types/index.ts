export interface CosConnection {
  id: string;
  name: string;
  secret_id: string;
  secret_key: string;
  region: string;
  provider: string;
  created_at: number;
}

export interface ObjectItem {
  key: string;
  size: number;
  lastModified: string;
  isDir: boolean;
}

export interface ObjectListResult {
  items: ObjectItem[];
  prefixes: string[];
  isTruncated: boolean;
  nextMarker: string;
}

export const providerLabels: Record<string, string> = {
  cos: "Tencent COS",
  s3: "AWS S3",
  oss: "Aliyun OSS",
  obs: "Huawei OBS",
  minio: "MinIO",
};
