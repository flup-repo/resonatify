import { Edit3, Play, StopCircle, Trash2 } from 'lucide-react';

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
        'group relative rounded-xl border border-border/60 bg-card p-5 shadow-sm transition-all duration-200 hover:border-border hover:shadow-md',
        isTesting && 'ring-2 ring-primary/50',
        !schedule.enabled && 'opacity-60',
      )}
    >
      {/* Header */}
      <div className="flex items-start justify-between gap-4">
        <div className="min-w-0 flex-1">
          <h3 className="mb-1 truncate text-lg font-semibold leading-tight">{schedule.name}</h3>
          <p className="text-sm text-muted-foreground">{repeatLabel}</p>
        </div>
        <Switch
          checked={schedule.enabled}
          onCheckedChange={(checked) => onToggle(schedule.id, checked)}
          aria-label="Toggle schedule"
          className="shrink-0"
        />
      </div>

      {/* Time Display */}
      <div className="mt-5 flex items-baseline gap-4">
        <span className="text-3xl font-bold tracking-tight text-foreground">{timeLabel}</span>
        <span className="text-sm text-muted-foreground">
          <span className="font-medium">Vol:</span> {schedule.volume}%
        </span>
      </div>

      {/* Actions */}
      <div className="mt-6 flex items-center gap-2">
        <Button
          variant="secondary"
          size="sm"
          className="gap-1.5 transition-all"
          onClick={() => onEdit(schedule)}
        >
          <Edit3 className="h-3.5 w-3.5" />
          <span>Edit</span>
        </Button>
        <Button
          variant={isTesting ? 'destructive' : 'outline'}
          size="sm"
          className="gap-1.5 transition-all"
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
          variant="ghost"
          size="sm"
          className="ml-auto gap-1.5 text-muted-foreground transition-all hover:text-destructive"
          onClick={() => onDelete(schedule.id)}
        >
          <Trash2 className="h-3.5 w-3.5" />
          <span className="hidden sm:inline">Delete</span>
        </Button>
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
