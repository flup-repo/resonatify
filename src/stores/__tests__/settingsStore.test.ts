import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useSettingsStore } from '../settingsStore';
import { defaultSettings } from '@/types/settings';

// Mock Tauri invoke
const invokeMock = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: any[]) => invokeMock(...args),
}));

describe('settingsStore', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useSettingsStore.setState({
      settings: defaultSettings,
      isLoading: false,
      hasLoaded: false,
      error: undefined,
    });
  });

  it('fetchSettings should update state on success', async () => {
    const mockSettings = {
      theme: 'light',
      launch_at_login: true,
      minimize_to_tray: false,
      show_notifications: true,
      notification_sound: false,
      default_volume: 50,
      announcement_enabled: false,
      announcement_sound: 'spell',
    };

    invokeMock.mockResolvedValueOnce(mockSettings);

    await useSettingsStore.getState().fetchSettings();

    const state = useSettingsStore.getState();
    expect(invokeMock).toHaveBeenCalledWith('get_settings');
    expect(state.isLoading).toBe(false);
    expect(state.hasLoaded).toBe(true);
    expect(state.settings.theme).toBe('light');
    expect(state.settings.launchAtLogin).toBe(true);
    expect(state.settings.defaultVolume).toBe(50);
  });

  it('fetchSettings should handle error', async () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    invokeMock.mockRejectedValueOnce(new Error('Failed'));

    await useSettingsStore.getState().fetchSettings();

    const state = useSettingsStore.getState();
    expect(state.isLoading).toBe(false);
    expect(state.error).toBeDefined();
    consoleSpy.mockRestore();
  });

  it('updateSettings should optimistic update and call backend', async () => {
    const mockUpdated = {
      theme: 'dark',
      launch_at_login: false,
      minimize_to_tray: true,
      show_notifications: true,
      notification_sound: true,
      default_volume: 100,
      announcement_enabled: true,
      announcement_sound: 'spell',
    };

    invokeMock.mockResolvedValueOnce(mockUpdated);

    await useSettingsStore.getState().updateSettings({ defaultVolume: 100 });

    expect(useSettingsStore.getState().settings.defaultVolume).toBe(100);
    expect(invokeMock).toHaveBeenCalledWith('update_settings', {
      payload: { default_volume: 100 },
    });
  });

  it('toggleLaunchAtLogin should call set_launch_at_login', async () => {
    // Mock update_settings response (called internally)
    invokeMock.mockResolvedValueOnce({}); // set_launch_at_login
    invokeMock.mockResolvedValueOnce({ ...defaultSettings, launch_at_login: true }); // update_settings

    await useSettingsStore.getState().toggleLaunchAtLogin(true);

    expect(invokeMock).toHaveBeenCalledWith('set_launch_at_login', { enabled: true });
    expect(invokeMock).toHaveBeenCalledWith('update_settings', expect.anything());
  });
});
