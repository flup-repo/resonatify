import { useState, useEffect } from 'react';

import { ScheduleList } from '@/components/ScheduleList';
import { ScheduleModal } from '@/components/ScheduleModal';
import { SettingsPanel } from '@/components/settings/SettingsPanel';
import { Button } from '@/components/ui';
import { useScheduleShortcuts } from '@/hooks/useScheduleShortcuts';
import { useScheduleStore } from '@/stores/scheduleStore';
import { useSettingsStore } from '@/stores/settingsStore';
import { useThemeSync } from '@/hooks/useThemeSync';

type Tab = 'schedules' | 'settings';

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('schedules');
  const [isInitializing, setIsInitializing] = useState(true);
  const openCreateModal = useScheduleStore((state) => state.openCreateModal);
  const fetchSettings = useSettingsStore((state) => state.fetchSettings);

  useScheduleShortcuts(openCreateModal);
  useThemeSync();

  // Load settings on mount and ensure proper initialization
  useEffect(() => {
    const initialize = async () => {
      try {
        await fetchSettings();
        // Small delay to ensure theme is applied and everything is rendered
        await new Promise(resolve => setTimeout(resolve, 50));
      } catch (error) {
        console.error('Failed to initialize app:', error);
      } finally {
        setIsInitializing(false);
      }
    };

    initialize();
  }, [fetchSettings]);

  // Show loading state during initialization
  if (isInitializing) {
    return (
      <div className="flex min-h-screen items-center justify-center" style={{ backgroundColor: 'rgb(var(--color-background))' }}>
        <div className="flex flex-col items-center gap-4">
          <div className="h-12 w-12 animate-spin rounded-full border-4 border-primary/30 border-t-primary"></div>
          <p className="text-sm text-muted-foreground">Loading Resonatify...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen flex-col" style={{ backgroundColor: 'rgb(var(--color-background))', color: 'rgb(var(--color-foreground))' }}>
      {/* App Header - Compact */}
      <header className="sticky top-0 z-10 border-b backdrop-blur-sm" style={{ borderColor: 'rgba(var(--color-border), 0.5)', backgroundColor: 'rgba(var(--color-card), 0.95)' }}>
        <div className="mx-auto flex max-w-4xl items-center justify-between px-8 py-3">
          <div className="flex items-center gap-2.5">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg" style={{ background: 'linear-gradient(to bottom right, rgba(var(--color-primary), 0.2), rgba(var(--color-primary), 0.1))' }}>
              <span className="text-base font-bold" style={{ color: 'rgb(var(--color-primary))' }}>R</span>
            </div>
            <h1 className="text-base font-bold leading-none tracking-tight">Resonatify</h1>
          </div>

          <nav className="inline-flex items-center gap-1 rounded-full border p-1 shadow-sm" style={{ borderColor: 'rgba(var(--color-border), 0.6)', backgroundColor: 'rgba(var(--color-background), 0.5)' }}>
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

      {/* Main Content - Centered with generous spacing */}
      <main className="flex grow justify-center overflow-hidden px-8 py-12">
        <div className="h-full w-full max-w-4xl overflow-y-auto">
          {activeTab === 'schedules' ? <ScheduleList /> : <SettingsPanel />}
        </div>
      </main>

      <ScheduleModal />
    </div>
  );
}

export default App;
