import { useState } from 'react';
import { api } from '../services/tauriApi';
import type { DuplicateGroup } from '../types';

const fmt = (b: number) => `${(b / 1024 / 1024).toFixed(1)} MB`;

export function DuplicatePage() {
  const [root, setRoot] = useState('D:\\');
  const [groups, setGroups] = useState<DuplicateGroup[]>([]);
  const [selected, setSelected] = useState<Record<string, boolean>>({});

  const scan = async () => {
    const result = await api.findDuplicates(root, 5 * 1024 * 1024);
    setGroups(result);
    setSelected({});
  };

  const autoSelect = () => {
    const next: Record<string, boolean> = {};
    groups.forEach((g) => {
      const sorted = [...g.files].sort((a, b) => (a.modified < b.modified ? 1 : -1));
      sorted.slice(1).forEach((f) => (next[f.path] = true));
    });
    setSelected(next);
  };

  const remove = async () => {
    if (!confirm('重复文件删除存在风险，是否继续？')) return;
    for (const path of Object.keys(selected).filter((k) => selected[k])) {
      await api.safeDelete(path);
    }
    scan();
  };

  return (
    <>
      <h1>重复文件检测</h1>
      <div className="toolbar">
        <input value={root} onChange={(e) => setRoot(e.target.value)} placeholder="输入盘符或目录" />
        <button onClick={scan}>开始检测</button>
        <button onClick={autoSelect}>保留最新，勾选其余</button>
        <button className="danger" onClick={remove}>
          删除勾选
        </button>
      </div>
      {groups.map((g) => (
        <div key={g.signature} className="group-card">
          <h4>重复组（可节省 {fmt(g.total_size)}）</h4>
          {g.files.map((f) => (
            <label key={f.path} className="row-check">
              <input
                type="checkbox"
                checked={!!selected[f.path]}
                onChange={(e) => setSelected((s) => ({ ...s, [f.path]: e.target.checked }))}
              />
              <span>{f.path}</span>
            </label>
          ))}
        </div>
      ))}
    </>
  );
}
