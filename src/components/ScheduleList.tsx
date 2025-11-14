import { useEffect } from 'react';

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
      return <div className="rounded-lg border border-dashed p-8 text-center">Loading schedules…</div>;
    }

    if (error) {
      return (
        <div className="rounded-lg border border-destructive/30 bg-destructive/5 p-6 text-center">
          <p className="text-sm text-destructive">{error}</p>
          <Button className="mt-3" onClick={fetchSchedules} size="sm">
            Retry
          </Button>
        </div>
      );
    }

    if (schedules.length === 0) {
      return (
        <div className="rounded-lg border border-dashed p-8 text-center">
          <p className="text-lg font-medium">No schedules yet</p>
          <p className="text-sm text-muted-foreground">Create your first reminder to get started.</p>
          <Button className="mt-4" onClick={openCreateModal}>
            New schedule
          </Button>
        </div>
      );
    }

    return (
      <div className="grid gap-4 md:grid-cols-2">
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
    <section className="space-y-6">
      <ScheduleHeader total={schedules.length} onAdd={openCreateModal} />
      {content}
    </section>
  );
}
