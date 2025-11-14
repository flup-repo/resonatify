import { invoke } from '@tauri-apps/api/core';
import { devtools } from 'zustand/middleware';
import { create } from 'zustand';

import type { CreateScheduleInput, Schedule, UpdateScheduleInput } from '@/types/schedule';
import { fromBackendRepeatType, toBackendRepeatType } from '@/types/repeat';

type ModalMode = 'create' | 'edit';

interface ScheduleStoreState {
  schedules: Schedule[];
  isLoading: boolean;
  isModalOpen: boolean;
  modalMode: ModalMode;
  editingSchedule?: Schedule;
  error?: string;
  testScheduleId?: string;
  isTestLoading: boolean;
}

interface ScheduleStoreActions {
  fetchSchedules: () => Promise<void>;
  openCreateModal: () => void;
  openEditModal: (schedule: Schedule) => void;
  closeModal: () => void;
  createSchedule: (input: CreateScheduleInput) => Promise<void>;
  updateSchedule: (id: string, input: UpdateScheduleInput) => Promise<void>;
  deleteSchedule: (id: string) => Promise<void>;
  toggleSchedule: (id: string, enabled: boolean) => Promise<void>;
  playTest: (schedule: Schedule) => Promise<void>;
  stopTest: () => Promise<void>;
  isTesting: (id: string) => boolean;
}

const mapSchedule = (payload: any): Schedule => ({
  id: payload.id,
  name: payload.name,
  audioFilePath: payload.audio_file_path ?? payload.audioFilePath,
  scheduledTime: payload.scheduled_time ?? payload.scheduledTime,
  enabled: payload.enabled,
  repeatType: fromBackendRepeatType(payload.repeat_type ?? payload.repeatType),
  volume: payload.volume,
  createdAt: payload.created_at ?? payload.createdAt,
  updatedAt: payload.updated_at ?? payload.updatedAt,
});

export const useScheduleStore = create<ScheduleStoreState & ScheduleStoreActions>()(
  devtools((set, get) => ({
    schedules: [],
    isLoading: false,
    isModalOpen: false,
    modalMode: 'create',
    testScheduleId: undefined,
    isTestLoading: false,

    fetchSchedules: async () => {
      try {
        set({ isLoading: true, error: undefined });
        const data = await invoke<any[]>('get_all_schedules');
        set({ schedules: data.map(mapSchedule), isLoading: false });
      } catch (error) {
        console.error('Failed to fetch schedules', error);
        set({ error: 'Failed to load schedules', isLoading: false });
      }
    },

    openCreateModal: () =>
      set({ isModalOpen: true, modalMode: 'create', editingSchedule: undefined }),

    openEditModal: (schedule) =>
      set({ isModalOpen: true, modalMode: 'edit', editingSchedule: schedule }),

    closeModal: () => set({ isModalOpen: false, editingSchedule: undefined }),

    createSchedule: async (input) => {
      set({ error: undefined });
      try {
        const payload = {
          ...input,
          audio_file_path: input.audioFilePath,
          scheduled_time: input.scheduledTime,
          repeat_type: toBackendRepeatType(input.repeatType),
        };
        const created = await invoke<any>('create_schedule', { input: payload });
        set((state) => ({ schedules: [...state.schedules, mapSchedule(created)] }));
      } catch (error) {
        console.error('Failed to create schedule', error);
        set({ error: 'Failed to create schedule' });
        throw error;
      }
    },

    updateSchedule: async (id, input) => {
      set({ error: undefined });
      try {
        const payload: Record<string, unknown> = {};
        if (input.name !== undefined) payload.name = input.name;
        if (input.audioFilePath !== undefined) payload.audio_file_path = input.audioFilePath;
        if (input.scheduledTime !== undefined) payload.scheduled_time = input.scheduledTime;
        if (input.enabled !== undefined) payload.enabled = input.enabled;
        if (input.repeatType !== undefined) payload.repeat_type = toBackendRepeatType(input.repeatType);
        if (input.volume !== undefined) payload.volume = input.volume;

        const updated = await invoke<any>('update_schedule', { id, input: payload });
        set((state) => ({
          schedules: state.schedules.map((schedule) =>
            schedule.id === id ? mapSchedule(updated) : schedule,
          ),
        }));
      } catch (error) {
        console.error('Failed to update schedule', error);
        set({ error: 'Failed to update schedule' });
        throw error;
      }
    },

    deleteSchedule: async (id) => {
      set({ error: undefined });
      try {
        await invoke('delete_schedule', { id });
        set((state) => ({
          schedules: state.schedules.filter((schedule) => schedule.id !== id),
        }));
      } catch (error) {
        console.error('Failed to delete schedule', error);
        set({ error: 'Failed to delete schedule' });
        throw error;
      }
    },

    toggleSchedule: async (id, enabled) => {
      set({ error: undefined });
      const previous = get().schedules;

      set((state) => ({
        schedules: state.schedules.map((schedule) =>
          schedule.id === id ? { ...schedule, enabled } : schedule,
        ),
      }));

      try {
        const updated = await invoke<any>('toggle_schedule_enabled', { id, enabled });
        set((state) => ({
          schedules: state.schedules.map((schedule) =>
            schedule.id === id ? mapSchedule(updated) : schedule,
          ),
        }));
      } catch (error) {
        console.error('Failed to toggle schedule', error);
        set({ error: 'Failed to toggle schedule', schedules: previous });
      }
    },

    playTest: async (schedule) => {
      const { stopTest, testScheduleId } = get();
      if (testScheduleId && testScheduleId !== schedule.id) {
        await stopTest();
      }

      try {
        set({ isTestLoading: true, testScheduleId: schedule.id, error: undefined });
        await invoke('play_audio_file', {
          path: schedule.audioFilePath,
          volume: schedule.volume,
        });
        set({ isTestLoading: false });
      } catch (error) {
        console.error('Failed to play audio test', error);
        set({ error: 'Failed to play audio test', isTestLoading: false, testScheduleId: undefined });
      }
    },

    stopTest: async () => {
      try {
        set({ isTestLoading: true });
        await invoke('stop_audio');
      } catch (error) {
        console.error('Failed to stop audio test', error);
        set({ error: 'Failed to stop audio test' });
      } finally {
        set({ isTestLoading: false, testScheduleId: undefined });
      }
    },

    isTesting: (id) => get().testScheduleId === id,
  })),
);
