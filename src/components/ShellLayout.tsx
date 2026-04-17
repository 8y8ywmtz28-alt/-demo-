import type { ReactNode } from 'react';
import { useAppStore } from '../stores/appStore';
import type { NavKey } from '../types';

const navs: Array<{ key: NavKey; label: string }> = [
  { key: 'dashboard', label: '磁盘总览' },
  { key: 'junk', label: '垃圾扫描' },
  { key: 'large', label: '大文件查找' },
  { key: 'duplicate', label: '重复文件' },
  { key: 'startup', label: '启动项管理' },
  { key: 'storage', label: '存储分析' },
  { key: 'tools', label: '实用工具' },
  { key: 'settings', label: '设置' }
];

export function ShellLayout({ children }: { children: ReactNode }) {
  const { nav, setNav, settings } = useAppStore();
  return (
    <div className={`app-shell ${settings.theme}`}>
      <aside className="sidebar">
        <h2>WinCleaner Pro</h2>
        {navs.map((n) => (
          <button key={n.key} onClick={() => setNav(n.key)} className={nav === n.key ? 'active' : ''}>
            {n.label}
          </button>
        ))}
      </aside>
      <main className="content">{children}</main>
    </div>
  );
}
