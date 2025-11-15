import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Controller, useForm } from 'react-hook-form';

import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Switch,
} from '@/components/ui';
import { useScheduleStore } from '@/stores/scheduleStore';
import type { CreateScheduleInput, RepeatType } from '@/types/schedule';
import { WEEKDAY_LABELS, formatRepeatType } from '@/types/repeat';

type FormValues = {
  name: string;
  audioFilePath: string;
  scheduledTime: string;
  enabled: boolean;
  repeatType: RepeatType['type'];
  weeklyDays: number[];
  intervalMinutes: number;
  volume: number;
};

const defaultValues: FormValues = {
  name: '',
  audioFilePath: '',
  scheduledTime: '07:00',
  enabled: true,
  repeatType: 'daily',
  weeklyDays: [],
  intervalMinutes: 60,
  volume: 80,
};

export function ScheduleModal() {
  const {
    isModalOpen,
    modalMode,
    closeModal,
    createSchedule,
    updateSchedule,
    editingSchedule,
  } = useScheduleStore();

  const { register, control, handleSubmit, reset, watch, setValue } = useForm<FormValues>({ defaultValues });

  const repeatType = watch('repeatType');
  const weeklyDays = watch('weeklyDays');

  useEffect(() => {
    if (editingSchedule) {
      const repeat = editingSchedule.repeatType;
      reset({
        name: editingSchedule.name,
        audioFilePath: editingSchedule.audioFilePath,
        scheduledTime: editingSchedule.scheduledTime,
        enabled: editingSchedule.enabled,
        repeatType: repeat.type,
        weeklyDays: repeat.type === 'weekly' ? repeat.days : [],
        intervalMinutes: repeat.type === 'custom' ? repeat.intervalMinutes : 60,
        volume: editingSchedule.volume,
      });
    } else {
      reset(defaultValues);
    }
  }, [editingSchedule, reset, isModalOpen]);

  const onSubmit = handleSubmit(async (values) => {
    const payload: CreateScheduleInput = {
      name: values.name,
      audioFilePath: values.audioFilePath,
      scheduledTime: values.scheduledTime,
      enabled: values.enabled,
      repeatType: (() => {
        switch (values.repeatType) {
          case 'once':
            return { type: 'once' } as RepeatType;
          case 'daily':
            return { type: 'daily' } as RepeatType;
          case 'weekdays':
            return { type: 'weekdays' } as RepeatType;
          case 'weekends':
            return { type: 'weekends' } as RepeatType;
          case 'custom':
            return { type: 'custom', intervalMinutes: values.intervalMinutes } as RepeatType;
          case 'weekly':
            return { type: 'weekly', days: values.weeklyDays } as RepeatType;
          default:
            return { type: 'daily' } as RepeatType;
        }
      })(),
      volume: values.volume,
    };

    if (modalMode === 'edit' && editingSchedule) {
      await updateSchedule(editingSchedule.id, payload);
    } else {
      await createSchedule(payload);
    }

    closeModal();
  });

  const handleFileSelect = async () => {
    try {
      const selection = await invoke<string | null>('open_audio_file_dialog');
      if (selection) {
        setValue('audioFilePath', selection, { shouldValidate: true });
      }
    } catch (error) {
      console.warn('Audio file picker requires Tauri support.', error);
    }
  };

  const repeatHint = (() => {
    if (repeatType === 'weekly' && weeklyDays.length) {
      return `Runs on ${formatRepeatType({ type: 'weekly', days: weeklyDays })}`;
    }
    if (repeatType === 'custom') {
      return `Repeats every ${watch('intervalMinutes')} minutes`;
    }
    return formatRepeatType({ type: repeatType as RepeatType['type'] } as RepeatType);
  })();

  return (
    <Dialog open={isModalOpen} onOpenChange={closeModal}>
      <DialogContent className="max-h-[90vh] max-w-2xl p-0">
        <div className="flex max-h-[90vh] flex-col bg-card">
          <DialogHeader className="shrink-0 bg-card px-6 pt-6 pb-4">
            <DialogTitle className="text-2xl">
              {modalMode === 'edit' ? 'Edit schedule' : 'Create schedule'}
            </DialogTitle>
            <DialogDescription className="text-base">
              Configure when and how the audio reminder should run.
            </DialogDescription>
          </DialogHeader>

          <form className="flex min-h-0 flex-col bg-card" onSubmit={onSubmit}>
            <div className="flex-1 space-y-5 overflow-y-auto bg-card px-6 pb-4">
          <div className="space-y-2">
            <Label htmlFor="name">Name</Label>
            <Input id="name" placeholder="Morning meditation" {...register('name', { required: true })} />
          </div>

          <div className="space-y-2">
            <Label htmlFor="audioFile">Audio file</Label>
            <div className="flex gap-3">
              <Input id="audioFile" placeholder="/Users/..." {...register('audioFilePath', { required: true })} />
              <Button type="button" variant="secondary" onClick={handleFileSelect}>
                Browse
              </Button>
            </div>
          </div>

          <div className="grid gap-4 sm:grid-cols-2">
            <div className="space-y-2">
              <Label htmlFor="scheduledTime">Time</Label>
              <Input id="scheduledTime" type="time" step={60} {...register('scheduledTime', { required: true })} />
            </div>

            <div className="space-y-2">
              <Label>Enabled</Label>
              <div className="flex h-10 items-center gap-3 rounded-md border px-3">
                <Switch checked={watch('enabled')} onCheckedChange={(checked) => setValue('enabled', checked)} />
                <span className="text-sm text-muted-foreground">{watch('enabled') ? 'Active' : 'Paused'}</span>
              </div>
            </div>
          </div>

          <div className="space-y-2">
            <Label>Repeat</Label>
            <Controller
              control={control}
              name="repeatType"
              render={({ field }) => (
                <Select onValueChange={field.onChange} value={field.value}>
                  <SelectTrigger>
                    <SelectValue placeholder="Select repeat" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="once">Once</SelectItem>
                    <SelectItem value="daily">Daily</SelectItem>
                    <SelectItem value="weekdays">Weekdays</SelectItem>
                    <SelectItem value="weekends">Weekends</SelectItem>
                    <SelectItem value="weekly">Weekly</SelectItem>
                    <SelectItem value="custom">Custom interval</SelectItem>
                  </SelectContent>
                </Select>
              )}
            />
            <p className="text-xs text-muted-foreground">{repeatHint}</p>

            {repeatType === 'weekly' && (
              <div className="mt-3 flex flex-wrap gap-2">
                {WEEKDAY_LABELS.map((label, index) => {
                  const isSelected = weeklyDays.includes(index);
                  return (
                    <button
                      type="button"
                      key={label}
                      className={`rounded-full border px-3 py-1 text-sm ${
                        isSelected ? 'border-primary bg-primary/10 text-primary' : 'text-muted-foreground'
                      }`}
                      onClick={() => {
                        setValue(
                          'weeklyDays',
                          isSelected ? weeklyDays.filter((day) => day !== index) : [...weeklyDays, index],
                        );
                      }}
                    >
                      {label}
                    </button>
                  );
                })}
              </div>
            )}

            {repeatType === 'custom' && (
              <div className="mt-3 space-y-2">
                <Label htmlFor="interval">Interval (minutes)</Label>
                <Input id="interval" type="number" min={1} {...register('intervalMinutes', { valueAsNumber: true })} />
              </div>
            )}
          </div>

          <div className="space-y-2">
            <Label htmlFor="volume">Volume ({watch('volume')}%)</Label>
            <Input id="volume" type="range" min={0} max={100} {...register('volume', { valueAsNumber: true })} />
          </div>
            </div>

            <DialogFooter className="shrink-0 border-t bg-card px-6 py-4">
              <Button type="button" variant="ghost" onClick={closeModal}>
                Cancel
              </Button>
              <Button type="submit">{modalMode === 'edit' ? 'Save changes' : 'Create'}</Button>
            </DialogFooter>
          </form>
        </div>
      </DialogContent>
    </Dialog>
  );
}
