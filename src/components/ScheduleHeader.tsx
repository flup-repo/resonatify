import { Plus } from 'lucide-react';

import { Button } from '@/components/ui';

interface ScheduleHeaderProps {
  total: number;
  onAdd: () => void;
}

export function ScheduleHeader({ total, onAdd }: ScheduleHeaderProps) {
  return (
    <header className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <p className="text-sm text-muted-foreground">Schedules</p>
        <h1 className="text-3xl font-semibold tracking-tight">Manage audio reminders</h1>
        <p className="text-sm text-muted-foreground">{total} scheduled items</p>
      </div>

      <div className="flex items-center gap-3">
        <span className="rounded-full bg-secondary/60 px-3 py-1 text-xs text-muted-foreground">
          ⌘ + N
        </span>
        <Button onClick={onAdd} className="gap-2">
          <Plus className="h-4 w-4" /> New schedule
        </Button>
      </div>
    </header>
  );
}
