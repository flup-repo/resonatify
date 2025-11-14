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
        'rounded-xl border border-border/80 bg-card/60 p-4 shadow-sm transition hover:shadow-lg',
        !schedule.enabled && 'opacity-70',
      )}
    >
      <div className="flex items-start justify-between gap-3">
        <div>
          <h3 className="text-lg font-semibold leading-tight">{schedule.name}</h3>
          <p className="text-sm text-muted-foreground">{repeatLabel}</p>
        </div>
        <Switch
          checked={schedule.enabled}
          onCheckedChange={(checked) => onToggle(schedule.id, checked)}
          aria-label="Toggle schedule"
        />
      </div>

      <div className="mt-4 flex items-center gap-4 text-sm text-muted-foreground">
        <span className="text-2xl font-semibold text-foreground">{timeLabel}</span>
        <span>Volume {schedule.volume}%</span>
      </div>

      <div className="mt-6 flex items-center gap-2 text-sm">
        <Button variant="secondary" size="sm" className="gap-1" onClick={() => onEdit(schedule)}>
          <Edit3 className="h-4 w-4" /> Edit
        </Button>
        <Button
          variant={isTesting ? 'destructive' : 'outline'}
          size="sm"
          className="gap-1"
          onClick={() => (isTesting ? onStopTest() : onTest(schedule))}
          disabled={isTestLoading && !isTesting}
        >
          {isTesting ? (
            <>
              <StopCircle className="h-4 w-4" /> Stop
            </>
          ) : (
            <>
              <Play className="h-4 w-4" /> Test
            </>
          )}
        </Button>
        <Button
          variant="ghost"
          size="sm"
          className="ml-auto text-destructive hover:text-destructive"
          onClick={() => onDelete(schedule.id)}
        >
          <Trash2 className="h-4 w-4" /> Delete
        </Button>
      </div>
    </article>
  );
}
