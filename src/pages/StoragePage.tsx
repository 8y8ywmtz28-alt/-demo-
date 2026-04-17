import { useState } from 'react';
import { api } from '../services/tauriApi';
import type { StorageNode } from '../types';

const fmt = (b: number) => `${(b / 1024 / 1024 / 1024).toFixed(2)} GB`;

function NodeView({ node, level = 0 }: { node: StorageNode; level?: number }) {
  const [open, setOpen] = useState(level < 1);
  return (
    <div className="tree-node" style={{ marginLeft: level * 16 }}>
      <div className="tree-row" onClick={() => setOpen((v) => !v)}>
        <span>{open ? '▾' : '▸'} {node.name || node.path}</span>
        <strong>{fmt(node.size)}</strong>
      </div>
      {open && node.children.map((c) => <NodeView key={c.path} node={c} level={level + 1} />)}
    </div>
  );
}

export function StoragePage() {
  const [root, setRoot] = useState('C:\\');
  const [node, setNode] = useState<StorageNode | null>(null);

  return (
    <>
      <h1>存储分析</h1>
      <div className="toolbar">
        <input value={root} onChange={(e) => setRoot(e.target.value)} placeholder="输入盘符" />
        <button onClick={async () => setNode(await api.analyzeStorage(root, 3))}>分析</button>
      </div>
      {node && <NodeView node={node} />}
    </>
  );
}
