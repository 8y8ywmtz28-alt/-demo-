import { useEffect, useState } from 'react';
import { api } from '../services/tauriApi';
import { useAppStore } from '../stores/appStore';
import { DiskCard } from '../components/DiskCard';
import { Section } from '../components/Section';

export function DashboardPage() {
  const { disks, setDisks, setSelectedDisk, setNav } = useAppStore();
  const [error, setError] = useState('');

  useEffect(() => {
    api
      .listDisks()
      .then(setDisks)
      .catch((e) => setError(String(e)));
  }, [setDisks]);

  return (
    <>
      <h1>磁盘总览</h1>
      {error && <div className="alert">读取磁盘信息失败：{error}</div>}
      <Section title="本机磁盘">
        <div className="disk-grid">
          {disks.map((disk) => (
            <DiskCard
              key={disk.mount_point}
              disk={disk}
              onClick={() => {
                setSelectedDisk(disk.mount_point);
                setNav('junk');
              }}
            />
          ))}
        </div>
      </Section>
      <Section title="推荐操作">
        <ul className="tips-list">
          <li>先执行“垃圾扫描”，优先清理临时文件与缩略图缓存。</li>
          <li>定期使用“大文件查找”，重点关注下载目录和桌面目录。</li>
          <li>启动项过多会拖慢开机速度，建议每周检查一次。</li>
        </ul>
      </Section>
    </>
  );
}
