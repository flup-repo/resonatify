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
      <div className="border-b border-border bg-card">
        <div className="mx-auto flex max-w-5xl items-center justify-between px-6 py-6">
          <div>
            <p className="text-sm text-muted-foreground">Resonatify</p>
            <h1 className="text-2xl font-semibold tracking-tight">Audio reminders</h1>
          </div>

          <div className="flex items-center gap-2 rounded-full border border-border bg-background p-1 text-sm">
            <Button
              variant={activeTab === 'schedules' ? 'default' : 'ghost'}
              size="sm"
              onClick={() => setActiveTab('schedules')}
              className="rounded-full"
            >
              Schedules
            </Button>
            <Button
              variant={activeTab === 'settings' ? 'default' : 'ghost'}
              size="sm"
              onClick={() => setActiveTab('settings')}
              className="rounded-full"
            >
              Settings
            </Button>
          </div>
        </div>
      </div>

      <div className="flex grow justify-center px-6 py-10 overflow-hidden">
        <div className="h-full w-full max-w-5xl overflow-y-auto">
          {activeTab === 'schedules' ? <ScheduleList /> : <SettingsPanel />}
        </div>
      </div>
      <ScheduleModal />
    </div>
  );
}

export default App;
