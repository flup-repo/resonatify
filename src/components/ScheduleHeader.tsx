import { Plus } from 'lucide-react';

import { Button } from '@/components/ui';

interface ScheduleHeaderProps {
  total: number;
  onAdd: () => void;
}

export function ScheduleHeader({ total, onAdd }: ScheduleHeaderProps) {
  return (
    <header className="flex flex-col gap-5 sm:flex-row sm:items-start sm:justify-between">
      <div className="space-y-1">
        <h1 className="text-3xl font-bold tracking-tight">Audio Schedules</h1>
        <p className="text-sm text-muted-foreground">
          {total === 0 ? 'No active reminders' : `${total} ${total === 1 ? 'reminder' : 'reminders'}`}
        </p>
      </div>

      <div className="flex items-center gap-3 sm:pt-1">
        <kbd className="hidden rounded-lg border border-border bg-muted px-2.5 py-1.5 text-xs font-medium text-muted-foreground shadow-sm sm:inline-block">
          ⌘N
        </kbd>
        <Button onClick={onAdd} size="lg" className="gap-2 shadow-sm">
          <Plus className="h-4 w-4" />
          <span>New schedule</span>
        </Button>
      </div>
    </header>
  );
}
