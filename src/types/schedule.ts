import type { BackendRepeatType, RepeatType } from './repeat';
import { fromBackendRepeatType } from './repeat';
export type { BackendRepeatType, RepeatType };

export interface Schedule {
  id: string;
  name: string;
  audioFilePath: string;
  scheduledTime: string; // HH:mm
  enabled: boolean;
  repeatType: RepeatType;
  volume: number;
  createdAt: string;
  updatedAt: string;
}

export interface CreateScheduleInput {
  name: string;
  audioFilePath: string;
  scheduledTime: string;
  enabled: boolean;
  repeatType: RepeatType;
  volume: number;
}

export type UpdateScheduleInput = Partial<CreateScheduleInput>;

export type BackendSchedule = {
  id: string;
  name: string;
  audio_file_path: string;
  scheduled_time: string;
  enabled: boolean;
  repeat_type: BackendRepeatType;
  volume: number;
  created_at: string;
  updated_at: string;
};

export function mapBackendSchedule(payload: BackendSchedule): Schedule {
  return {
    id: payload.id,
    name: payload.name,
    audioFilePath: payload.audio_file_path,
    scheduledTime: payload.scheduled_time,
    enabled: payload.enabled,
    repeatType: fromBackendRepeatType(payload.repeat_type),
    volume: payload.volume,
    createdAt: payload.created_at,
    updatedAt: payload.updated_at,
  };
}

export function formatTimeLabel(value: string) {
  const [hours, minutes] = value.split(':').map(Number);
  if (Number.isNaN(hours) || Number.isNaN(minutes)) return value;

  const period = hours >= 12 ? 'PM' : 'AM';
  const hour12 = hours % 12 || 12;
  return `${hour12}:${minutes.toString().padStart(2, '0')} ${period}`;
}