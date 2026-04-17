import { useState } from 'react';
import { api } from '../services/tauriApi';
import type { ScanCategory } from '../types';
import { Section } from '../components/Section';
import { useAppStore } from '../stores/appStore';

const fmt = (v: number) => `${(v / 1024 / 1024).toFixed(2)} MB`;

export function JunkScanPage() {
  const { selectedDisk } = useAppStore();
  const [items, setItems] = useState<ScanCategory[]>([]);
  const [selected, setSelected] = useState<Record<string, boolean>>({});
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const run = async () => {
    setLoading(true);
    setMessage(`正在扫描${selectedDisk ? `（目标盘：${selectedDisk}）` : ''}，请稍候...`);
    try {
      const r = await api.scanJunk(selectedDisk);
      setItems(r);
      const initial: Record<string, boolean> = {};
      r.forEach((x) => (initial[x.key] = x.default_selected));
      setSelected(initial);
      setMessage(`扫描完成，共识别 ${r.length} 类可处理项。`);
    } catch (e) {
      setMessage(`扫描失败：${String(e)}`);
    } finally {
      setLoading(false);
    }
  };

  const clean = async () => {
    const keys = Object.entries(selected)
      .filter(([, v]) => v)
      .map(([k]) => k);
    if (!keys.length) return setMessage('请先勾选需要清理的项目。');
    if (!confirm('即将执行清理。系统关键目录已保护，但仍建议关闭相关软件后继续。')) return;
    const result = await api.cleanupSelected(keys);
    setMessage(`已处理 ${result.length} 项。`);
    run();
  };

  return (
    <>
      <h1>垃圾文件扫描</h1>
      <div className="toolbar">
        <button onClick={run} disabled={loading}>
          {loading ? '扫描中...' : '开始扫描'}
        </button>
        <button onClick={clean}>执行清理</button>
      </div>
      {message && <div className="alert">{message}</div>}
      <Section title="扫描结果">
        <table className="table">
          <thead>
            <tr>
              <th>选择</th>
              <th>类别</th>
              <th>文件数</th>
              <th>体积</th>
              <th>风险</th>
              <th>路径示例</th>
            </tr>
          </thead>
          <tbody>
            {items.map((x) => (
              <tr key={x.key}>
                <td>
                  <input
                    type="checkbox"
                    checked={!!selected[x.key]}
                    onChange={(e) => setSelected((s) => ({ ...s, [x.key]: e.target.checked }))}
                  />
                </td>
                <td>{x.name}</td>
                <td>{x.file_count}</td>
                <td>{fmt(x.total_size)}</td>
                <td>{x.risk_level}</td>
                <td>{x.sample_paths[0] ?? '-'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </Section>
    </>
  );
}
