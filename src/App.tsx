import { useState } from 'react';

import { ScheduleList } from '@/components/ScheduleList';
import { ScheduleModal } from '@/components/ScheduleModal';
import { SettingsPanel } from '@/components/settings/SettingsPanel';
import { Button } from '@/components/ui';
import { useScheduleShortcuts } from '@/hooks/useScheduleShortcuts';
import { useScheduleStore } from '@/stores/scheduleStore';
import { useThemeSync } from '@/hooks/useThemeSync';

type Tab = 'schedules' | 'settings';

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('schedules');
  const openCreateModal = useScheduleStore((state) => state.openCreateModal);
  useScheduleShortcuts(openCreateModal);
  useThemeSync();

  return (
    <div className="flex min-h-screen flex-col bg-background text-foreground">
      {/* App Header */}
      <header className="sticky top-0 z-10 border-b border-border/50 bg-card/95 backdrop-blur-sm">
        <div className="mx-auto flex max-w-5xl items-center justify-between px-6 py-4">
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-primary/20 to-primary/10">
              <span className="text-lg font-bold text-primary">R</span>
            </div>
            <div>
              <h1 className="text-lg font-bold leading-none tracking-tight">Resonatify</h1>
              <p className="text-xs text-muted-foreground">Audio reminders</p>
            </div>
          </div>

          <nav className="inline-flex items-center gap-1 rounded-full border border-border/60 bg-background/50 p-1 shadow-sm">
            <Button
              variant={activeTab === 'schedules' ? 'default' : 'ghost'}
              size="sm"
              onClick={() => setActiveTab('schedules')}
              className="rounded-full px-4 transition-all"
            >
              Schedules
            </Button>
            <Button
              variant={activeTab === 'settings' ? 'default' : 'ghost'}
              size="sm"
              onClick={() => setActiveTab('settings')}
              className="rounded-full px-4 transition-all"
            >
              Settings
            </Button>
          </nav>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex grow justify-center overflow-hidden px-6 py-8">
        <div className="h-full w-full max-w-5xl overflow-y-auto">
          {activeTab === 'schedules' ? <ScheduleList /> : <SettingsPanel />}
        </div>
      </main>

      <ScheduleModal />
    </div>
  );
}

export default App;
