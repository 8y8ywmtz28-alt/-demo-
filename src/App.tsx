import { ShellLayout } from './components/ShellLayout';
import { DashboardPage } from './pages/DashboardPage';
import { DuplicatePage } from './pages/DuplicatePage';
import { JunkScanPage } from './pages/JunkScanPage';
import { LargeFilesPage } from './pages/LargeFilesPage';
import { SettingsPage } from './pages/SettingsPage';
import { StartupPage } from './pages/StartupPage';
import { StoragePage } from './pages/StoragePage';
import { ToolsPage } from './pages/ToolsPage';
import { useAppStore } from './stores/appStore';

function Router() {
  const nav = useAppStore((s) => s.nav);
  switch (nav) {
    case 'junk':
      return <JunkScanPage />;
    case 'large':
      return <LargeFilesPage />;
    case 'duplicate':
      return <DuplicatePage />;
    case 'startup':
      return <StartupPage />;
    case 'storage':
      return <StoragePage />;
    case 'tools':
      return <ToolsPage />;
    case 'settings':
      return <SettingsPage />;
    default:
      return <DashboardPage />;
  }
}

export default function App() {
  return (
    <ShellLayout>
      <Router />
    </ShellLayout>
  );
}
