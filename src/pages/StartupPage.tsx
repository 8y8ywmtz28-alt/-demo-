import { useEffect, useState } from 'react';
import { api } from '../services/tauriApi';
import type { StartupItem } from '../types';

export function StartupPage() {
  const [items, setItems] = useState<StartupItem[]>([]);
  const [msg, setMsg] = useState('');

  const load = () => api.listStartupItems().then(setItems).catch((e) => setMsg(String(e)));
  useEffect(load, []);

  return (
    <>
      <h1>启动项管理</h1>
      {msg && <div className="alert">{msg}</div>}
      <table className="table">
        <thead>
          <tr>
            <th>名称</th>
            <th>来源</th>
            <th>命令/路径</th>
            <th>状态</th>
            <th>建议</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          {items.map((x) => (
            <tr key={x.id}>
              <td>{x.name}</td>
              <td>{x.source}</td>
              <td>{x.command}</td>
              <td>{x.enabled ? '已启用' : '已禁用'}</td>
              <td>{x.recommendation === 'keep' ? '建议保留' : '可自行判断'}</td>
              <td>
                <button
                  disabled={!x.mutable}
                  onClick={async () => {
                    await api.setStartupStatus(x.id, !x.enabled);
                    load();
                  }}
                >
                  {x.enabled ? '禁用' : '启用'}
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}
