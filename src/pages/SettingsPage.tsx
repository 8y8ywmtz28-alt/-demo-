import { useAppStore } from '../stores/appStore';

export function SettingsPage() {
  const { settings, updateSettings } = useAppStore();
  return (
    <>
      <h1>设置</h1>
      <div className="form-grid">
        <label>
          最小大文件阈值（MB）
          <input
            type="number"
            value={settings.minLargeFileMB}
            onChange={(e) => updateSettings({ minLargeFileMB: Number(e.target.value) })}
          />
        </label>
        <label>
          扫描排除目录（分号分隔）
          <input
            value={settings.excludeDirs.join(';')}
            onChange={(e) => updateSettings({ excludeDirs: e.target.value.split(';').filter(Boolean) })}
          />
        </label>
        <label>
          主题
          <select value={settings.theme} onChange={(e) => updateSettings({ theme: e.target.value as 'light' | 'dark' })}>
            <option value="light">浅色</option>
            <option value="dark">深色</option>
          </select>
        </label>
        <label>
          <input
            type="checkbox"
            checked={settings.enableAnimation}
            onChange={(e) => updateSettings({ enableAnimation: e.target.checked })}
          />
          启用细微动画
        </label>
        <label>
          <input
            type="checkbox"
            checked={settings.riskWarning}
            onChange={(e) => updateSettings({ riskWarning: e.target.checked })}
          />
          显示风险提示
        </label>
      </div>
    </>
  );
}
