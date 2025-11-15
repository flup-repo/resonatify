import { Edit3, Play, StopCircle, Trash2, Music } from 'lucide-react';

import { Button, Switch } from '@/components/ui';
import { formatRepeatType } from '@/types/repeat';
import type { Schedule } from '@/types/schedule';
import { formatTimeLabel } from '@/types/schedule';
import { cn } from '@/utils/cn';

interface ScheduleCardProps {
  schedule: Schedule;
  onToggle: (id: string, enabled: boolean) => void;
  onEdit: (schedule: Schedule) => void;
  onDelete: (id: string) => void;
  onTest: (schedule: Schedule) => void;
  onStopTest: () => void;
  isTesting: boolean;
  isTestLoading: boolean;
}

export function ScheduleCard({
  schedule,
  onToggle,
  onEdit,
  onDelete,
  onTest,
  onStopTest,
  isTesting,
  isTestLoading,
}: ScheduleCardProps) {
  const repeatLabel = formatRepeatType(schedule.repeatType);
  const timeLabel = formatTimeLabel(schedule.scheduledTime);

  return (
    <article
      className={cn(
        'group relative flex min-h-[80px] items-start gap-4 rounded-xl border border-border bg-card px-8 py-6 shadow-sm transition-all duration-200 hover:border-primary/30 hover:shadow-md',
        isTesting && 'ring-2 ring-primary/50',
        !schedule.enabled && 'opacity-60',
      )}
    >
      {/* Icon */}
      <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary">
        <Music className="h-5 w-5" />
      </div>

      {/* Content */}
      <div className="flex min-w-0 flex-1 flex-col gap-1">
        <h3 className="truncate text-base font-semibold leading-tight">{schedule.name}</h3>
        <p className="text-sm text-muted-foreground">
          {repeatLabel} at {timeLabel} • Volume {schedule.volume}%
        </p>
      </div>

      {/* Actions - Right aligned */}
      <div className="flex shrink-0 items-center gap-3">
        <Button
          variant={isTesting ? 'destructive' : 'outline'}
          size="sm"
          className="gap-1.5"
          onClick={() => (isTesting ? onStopTest() : onTest(schedule))}
          disabled={isTestLoading && !isTesting}
        >
          {isTesting ? (
            <>
              <StopCircle className="h-3.5 w-3.5" />
              <span>Stop</span>
            </>
          ) : (
            <>
              <Play className="h-3.5 w-3.5" />
              <span>Test</span>
            </>
          )}
        </Button>
        <Button
          variant="secondary"
          size="sm"
          className="gap-1.5"
          onClick={() => onEdit(schedule)}
        >
          <Edit3 className="h-3.5 w-3.5" />
          <span>Edit</span>
        </Button>
        <Button
          variant="ghost"
          size="sm"
          className="gap-1.5 text-muted-foreground hover:text-destructive"
          onClick={() => onDelete(schedule.id)}
        >
          <Trash2 className="h-3.5 w-3.5" />
          <span className="hidden sm:inline">Delete</span>
        </Button>
        <Switch
          checked={schedule.enabled}
          onCheckedChange={(checked) => onToggle(schedule.id, checked)}
          aria-label="Toggle schedule"
          className="shrink-0"
        />
      </div>

      {/* Testing Indicator */}
      {isTesting && (
        <div className="absolute right-3 top-3 flex h-2 w-2">
          <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-primary opacity-75"></span>
          <span className="relative inline-flex h-2 w-2 rounded-full bg-primary"></span>
        </div>
      )}
    </article>
  );
}
