export type NavKey =
  | 'dashboard'
  | 'junk'
  | 'large'
  | 'duplicate'
  | 'startup'
  | 'storage'
  | 'tools'
  | 'settings';

export interface DiskInfo {
  mount_point: string;
  total_bytes: number;
  available_bytes: number;
  used_bytes: number;
  used_percent: number;
}

export interface ScanCategory {
  key: string;
  name: string;
  total_size: number;
  file_count: number;
  risk_level: 'safe' | 'warning' | 'high';
  sample_paths: string[];
  default_selected: boolean;
}

export interface LargeFileItem {
  path: string;
  name: string;
  size: number;
  modified: string;
}

export interface DuplicateGroup {
  signature: string;
  total_size: number;
  files: LargeFileItem[];
}

export interface StartupItem {
  id: string;
  name: string;
  source: string;
  command: string;
  enabled: boolean;
  recommendation: 'keep' | 'review';
  mutable: boolean;
}

export interface StorageNode {
  path: string;
  name: string;
  size: number;
  children: StorageNode[];
}

export interface RecentFile {
  path: string;
  name: string;
  size: number;
  modified: string;
}

export interface DesktopSuggestion {
  category: string;
  count: number;
  total_size: number;
}

export interface ChatCacheItem {
  name: string;
  path: string;
  size: number;
}

export interface AppSettings {
  excludeDirs: string[];
  minLargeFileMB: number;
  enableAnimation: boolean;
  riskWarning: boolean;
  language: 'zh-CN';
  theme: 'light' | 'dark';
}
