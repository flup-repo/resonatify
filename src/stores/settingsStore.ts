import { invoke } from '@tauri-apps/api/core';
import { devtools } from 'zustand/middleware';
import { create } from 'zustand';

import {
  defaultSettings,
  mapFromBackend,
  Settings,
  SettingsUpdateInput,
  toBackendPayload,
} from '@/types/settings';

interface SettingsStoreState {
  settings: Settings;
  isLoading: boolean;
  hasLoaded: boolean;
  error?: string;
}

interface SettingsStoreActions {
  fetchSettings: () => Promise<void>;
  ensureLoaded: () => Promise<void>;
  updateSettings: (input: SettingsUpdateInput) => Promise<void>;
  setTheme: (theme: Settings['theme']) => Promise<void>;
  toggleLaunchAtLogin: (value: boolean) => Promise<void>;
  toggleMinimizeToTray: (value: boolean) => Promise<void>;
  toggleNotifications: (value: boolean) => Promise<void>;
  toggleNotificationSound: (value: boolean) => Promise<void>;
  setDefaultVolume: (value: number) => Promise<void>;
}

export const useSettingsStore = create<SettingsStoreState & SettingsStoreActions>()(
  devtools((set, get) => ({
    settings: defaultSettings,
    isLoading: false,
    hasLoaded: false,
    error: undefined,

    fetchSettings: async () => {
      try {
        set({ isLoading: true, error: undefined });
        const data = await invoke('get_settings');
        set({
          settings: mapFromBackend(data as any),
          isLoading: false,
          hasLoaded: true,
        });
      } catch (error) {
        console.error('Failed to load settings', error);
        set({ error: 'Failed to load settings', isLoading: false });
      }
    },

    ensureLoaded: async () => {
      const { hasLoaded, isLoading, fetchSettings } = get();
      if (hasLoaded || isLoading) return;
      await fetchSettings();
    },

    updateSettings: async (input) => {
      const current = get().settings;
      const optimistic = { ...current, ...input };
      set({ settings: optimistic, error: undefined });

      try {
        const payload = toBackendPayload(input);
        const updated = await invoke('update_settings', { payload });
        set({ settings: mapFromBackend(updated as any) });
      } catch (error) {
        console.error('Failed to update settings', error);
        set({ settings: current, error: 'Failed to update settings' });
      }
    },

    setTheme: async (theme) => get().updateSettings({ theme }),
    toggleLaunchAtLogin: async (value) => {
      await invoke('set_launch_at_login', { enabled: value });
      await get().updateSettings({ launchAtLogin: value });
    },
    toggleMinimizeToTray: async (value) => get().updateSettings({ minimizeToTray: value }),
    toggleNotifications: async (value) => get().updateSettings({ showNotifications: value }),
    toggleNotificationSound: async (value) => get().updateSettings({ notificationSound: value }),
    setDefaultVolume: async (value) => get().updateSettings({ defaultVolume: value }),
  })),
);
