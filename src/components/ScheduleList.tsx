import { useEffect } from 'react';
import { Music, Loader2, AlertCircle } from 'lucide-react';

import { ScheduleCard } from '@/components/ScheduleCard';
import { ScheduleHeader } from '@/components/ScheduleHeader';
import { Button } from '@/components/ui';
import { useScheduleStore } from '@/stores/scheduleStore';

export function ScheduleList() {
  const {
    schedules,
    isLoading,
    error,
    fetchSchedules,
    openCreateModal,
    openEditModal,
    deleteSchedule,
    toggleSchedule,
    playTest,
    stopTest,
    isTesting,
    isTestLoading,
  } = useScheduleStore();

  useEffect(() => {
    fetchSchedules();
  }, [fetchSchedules]);

  const content = (() => {
    if (isLoading) {
      return (
        <div className="flex flex-col items-center justify-center rounded-xl border border-dashed border-border/50 bg-card/30 px-8 py-16">
          <Loader2 className="mb-4 h-10 w-10 animate-spin text-muted-foreground" />
          <p className="text-sm text-muted-foreground">Loading schedules…</p>
        </div>
      );
    }

    if (error) {
      return (
        <div className="flex flex-col items-center rounded-xl border border-destructive/30 bg-destructive/5 px-6 py-12 text-center">
          <AlertCircle className="mb-4 h-10 w-10 text-destructive" />
          <p className="mb-2 text-sm font-medium text-destructive">Failed to load schedules</p>
          <p className="mb-4 text-sm text-destructive/80">{error}</p>
          <Button variant="outline" onClick={fetchSchedules} size="sm">
            Try again
          </Button>
        </div>
      );
    }

    if (schedules.length === 0) {
      return (
        <div className="flex flex-col items-center rounded-xl border border-dashed border-border/50 bg-gradient-to-br from-card/30 to-card/10 px-8 py-16 text-center">
          <div className="mb-6 flex h-20 w-20 items-center justify-center rounded-full bg-primary/10">
            <Music className="h-10 w-10 text-primary" />
          </div>
          <h3 className="mb-2 text-xl font-semibold">No schedules yet</h3>
          <p className="mb-6 max-w-sm text-sm leading-relaxed text-muted-foreground">
            Create your first audio reminder to play sounds at specific times.
          </p>
          <Button onClick={openCreateModal} size="lg" className="gap-2">
            <Music className="h-4 w-4" /> Create your first schedule
          </Button>
          <p className="mt-4 text-xs text-muted-foreground">
            Example: Play meditation.mp3 every day at 7:00 AM
          </p>
        </div>
      );
    }

    return (
      <div className="grid gap-5 md:grid-cols-2">
        {schedules.map((schedule) => (
          <ScheduleCard
            key={schedule.id}
            schedule={schedule}
            onToggle={toggleSchedule}
            onEdit={openEditModal}
            onDelete={deleteSchedule}
            onTest={playTest}
            onStopTest={stopTest}
            isTesting={isTesting(schedule.id)}
            isTestLoading={isTestLoading}
          />
        ))}
      </div>
    );
  })();

  return (
    <section className="space-y-8">
      <ScheduleHeader total={schedules.length} onAdd={openCreateModal} />
      {content}
    </section>
  );
}
