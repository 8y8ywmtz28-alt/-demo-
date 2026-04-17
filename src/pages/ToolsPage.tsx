import { useState } from 'react';
import { api } from '../services/tauriApi';
import type { ChatCacheItem, DesktopSuggestion, RecentFile } from '../types';

const mb = (b: number) => `${(b / 1024 / 1024).toFixed(2)} MB`;

export function ToolsPage() {
  const [downloads, setDownloads] = useState<RecentFile[]>([]);
  const [desktop, setDesktop] = useState<DesktopSuggestion[]>([]);
  const [chatCaches, setChatCaches] = useState<ChatCacheItem[]>([]);
  const [quick, setQuick] = useState<Record<string, string>>({});
  const [msg, setMsg] = useState('');

  return (
    <>
      <h1>实用工具</h1>
      <div className="toolbar">
        <button onClick={async () => setMsg(await api.emptyRecycleBin())}>一键清空回收站</button>
        <button onClick={async () => setDownloads(await api.getRecentDownloads())}>最近下载</button>
        <button onClick={async () => setDesktop(await api.getDesktopSuggestions())}>桌面整理建议</button>
        <button onClick={async () => setChatCaches(await api.detectChatCaches())}>聊天缓存识别</button>
        <button onClick={async () => setQuick(await api.quickDirs())}>常用目录</button>
      </div>
      {msg && <div className="alert">{msg}</div>}
      <h3>最近下载</h3>
      <ul>{downloads.map((f) => <li key={f.path}>{f.name} - {mb(f.size)}</li>)}</ul>
      <h3>桌面整理建议</h3>
      <ul>{desktop.map((d) => <li key={d.category}>{d.category}: {d.count} 个，{mb(d.total_size)}</li>)}</ul>
      <h3>聊天缓存（仅提示）</h3>
      <ul>{chatCaches.map((c) => <li key={c.path}>{c.name} - {c.path} - {mb(c.size)}</li>)}</ul>
      <h3>常用目录快捷跳转</h3>
      <ul>
        {Object.entries(quick).map(([k, v]) => (
          <li key={k}>
            {k}：{v} <button onClick={() => api.openPath(v)}>打开</button>
          </li>
        ))}
      </ul>
    </>
  );
}
