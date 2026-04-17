import type { DiskInfo } from '../types';
import { ProgressLine } from './ProgressLine';

interface Props {
  disk: DiskInfo;
  onClick: () => void;
}

const gb = (v: number) => `${(v / 1024 / 1024 / 1024).toFixed(1)} GB`;

export function DiskCard({ disk, onClick }: Props) {
  return (
    <button className="disk-card" onClick={onClick}>
      <div className="disk-top">
        <strong>{disk.mount_point}</strong>
        <span>{disk.used_percent.toFixed(1)}%</span>
      </div>
      <ProgressLine value={disk.used_percent} />
      <div className="disk-meta">
        <span>已用 {gb(disk.used_bytes)}</span>
        <span>可用 {gb(disk.available_bytes)}</span>
        <span>总计 {gb(disk.total_bytes)}</span>
      </div>
    </button>
  );
}
