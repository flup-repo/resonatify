import { ScheduleList } from '@/components/ScheduleList';
import { ScheduleModal } from '@/components/ScheduleModal';
import { useScheduleShortcuts } from '@/hooks/useScheduleShortcuts';
import { useScheduleStore } from '@/stores/scheduleStore';

function App() {
  const openCreateModal = useScheduleStore((state) => state.openCreateModal);
  useScheduleShortcuts(openCreateModal);

  return (
    <div className="min-h-screen bg-background text-foreground">
      <div className="mx-auto max-w-5xl px-6 py-10">
        <ScheduleList />
      </div>
      <ScheduleModal />
    </div>
  );
}

export default App;
