import { create } from 'zustand';
import type { AppSettings, DiskInfo, NavKey } from '../types';

const defaultSettings: AppSettings = {
  excludeDirs: [],
  minLargeFileMB: 100,
  enableAnimation: true,
  riskWarning: true,
  language: 'zh-CN',
  theme: 'light'
};

interface AppState {
  nav: NavKey;
  disks: DiskInfo[];
  selectedDisk?: string;
  settings: AppSettings;
  setNav: (nav: NavKey) => void;
  setDisks: (disks: DiskInfo[]) => void;
  setSelectedDisk: (disk: string) => void;
  updateSettings: (partial: Partial<AppSettings>) => void;
}

const loadSettings = (): AppSettings => {
  const raw = localStorage.getItem('wincleaner_settings');
  if (!raw) return defaultSettings;
  try {
    return { ...defaultSettings, ...(JSON.parse(raw) as Partial<AppSettings>) };
  } catch {
    return defaultSettings;
  }
};

export const useAppStore = create<AppState>((set, get) => ({
  nav: 'dashboard',
  disks: [],
  settings: loadSettings(),
  setNav: (nav) => set({ nav }),
  setDisks: (disks) => set({ disks }),
  setSelectedDisk: (selectedDisk) => set({ selectedDisk }),
  updateSettings: (partial) => {
    const merged = { ...get().settings, ...partial };
    localStorage.setItem('wincleaner_settings', JSON.stringify(merged));
    set({ settings: merged });
  }
}));
