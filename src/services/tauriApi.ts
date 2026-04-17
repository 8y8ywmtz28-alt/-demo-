import { invoke } from '@tauri-apps/api/core';
import type {
  ChatCacheItem,
  DesktopSuggestion,
  DiskInfo,
  DuplicateGroup,
  LargeFileItem,
  RecentFile,
  ScanCategory,
  StartupItem,
  StorageNode
} from '../types';

export const api = {
  listDisks: () => invoke<DiskInfo[]>('list_disks'),
  scanJunk: (target?: string) => invoke<ScanCategory[]>('scan_junk', { target }),
  cleanupSelected: (keys: string[]) => invoke<string[]>('cleanup_selected', { keys }),
  scanLargeFiles: (root: string, minBytes: number) =>
    invoke<LargeFileItem[]>('scan_large_files', { root, minBytes }),
  findDuplicates: (root: string, minBytes: number) =>
    invoke<DuplicateGroup[]>('find_duplicates', { root, minBytes }),
  safeDelete: (path: string) => invoke<string>('safe_delete_file', { path }),
  listStartupItems: () => invoke<StartupItem[]>('list_startup_items'),
  setStartupStatus: (id: string, enable: boolean) =>
    invoke<void>('set_startup_status', { id, enable }),
  analyzeStorage: (root: string, depth = 2) =>
    invoke<StorageNode>('analyze_storage', { root, depth }),
  emptyRecycleBin: () => invoke<string>('empty_recycle_bin'),
  getRecentDownloads: () => invoke<RecentFile[]>('get_recent_downloads'),
  getDesktopSuggestions: () => invoke<DesktopSuggestion[]>('get_desktop_suggestions'),
  detectChatCaches: () => invoke<ChatCacheItem[]>('detect_chat_caches'),
  openPath: (path: string) => invoke<void>('open_in_explorer', { path }),
  quickDirs: () => invoke<Record<string, string>>('quick_dirs'),
  restoreFile: (path: string) => invoke<void>('restore_file', { path })
};
