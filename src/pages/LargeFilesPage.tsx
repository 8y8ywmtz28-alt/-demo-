import { useState } from 'react';
import { api } from '../services/tauriApi';
import type { LargeFileItem } from '../types';

const fmt = (b: number) => `${(b / 1024 / 1024).toFixed(1)} MB`;

export function LargeFilesPage() {
  const [root, setRoot] = useState('C:\\');
  const [threshold, setThreshold] = useState(100);
  const [items, setItems] = useState<LargeFileItem[]>([]);
  const [todo, setTodo] = useState<Set<string>>(new Set());

  const run = async () => {
    const res = await api.scanLargeFiles(root, threshold * 1024 * 1024);
    setItems(res);
  };

  return (
    <>
      <h1>大文件查找</h1>
      <div className="toolbar">
        <input value={root} onChange={(e) => setRoot(e.target.value)} placeholder="C:\\ 或 D:\\Data" />
        <select value={threshold} onChange={(e) => setThreshold(Number(e.target.value))}>
          <option value={100}>大于 100MB</option>
          <option value={500}>大于 500MB</option>
          <option value={1024}>大于 1GB</option>
        </select>
        <button onClick={run}>扫描</button>
      </div>
      <table className="table">
        <thead>
          <tr>
            <th>文件名</th>
            <th>路径</th>
            <th>大小</th>
            <th>修改时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          {items.map((x) => (
            <tr key={x.path}>
              <td>{x.name}</td>
              <td>{x.path}</td>
              <td>{fmt(x.size)}</td>
              <td>{x.modified}</td>
              <td>
                <button onClick={() => api.openPath(x.path)}>打开目录</button>
                <button
                  onClick={() =>
                    setTodo((s) => {
                      const n = new Set(s);
                      n.add(x.path);
                      return n;
                    })
                  }
                >
                  加入待处理
                </button>
                <button
                  className="danger"
                  onClick={async () => {
                    if (!confirm(`确认删除 ${x.name} ?\n文件将进入应用恢复区。`)) return;
                    await api.safeDelete(x.path);
                    setItems((arr) => arr.filter((i) => i.path !== x.path));
                  }}
                >
                  删除
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <p>待处理数量：{todo.size}</p>
    </>
  );
}
