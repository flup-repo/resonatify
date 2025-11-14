export type ThemePreference = 'light' | 'dark' | 'system';

export interface Settings {
  theme: ThemePreference;
  launchAtLogin: boolean;
  minimizeToTray: boolean;
  showNotifications: boolean;
  notificationSound: boolean;
  defaultVolume: number;
}

export type SettingsUpdateInput = Partial<Settings>;

export interface BackendSettingsResponse {
  theme: string;
  launch_at_login: boolean;
  minimize_to_tray: boolean;
  show_notifications: boolean;
  notification_sound: boolean;
  default_volume: number;
}

export interface BackendSettingsPayload {
  theme?: string;
  launch_at_login?: boolean;
  minimize_to_tray?: boolean;
  show_notifications?: boolean;
  notification_sound?: boolean;
  default_volume?: number;
}

export const defaultSettings: Settings = {
  theme: 'system',
  launchAtLogin: false,
  minimizeToTray: true,
  showNotifications: true,
  notificationSound: true,
  defaultVolume: 80,
};

export function mapFromBackend(payload: BackendSettingsResponse): Settings {
  return {
    theme: isThemePreference(payload.theme) ? payload.theme : 'system',
    launchAtLogin: payload.launch_at_login,
    minimizeToTray: payload.minimize_to_tray,
    showNotifications: payload.show_notifications,
    notificationSound: payload.notification_sound,
    defaultVolume: clampVolume(payload.default_volume),
  };
}

export function toBackendPayload(input: SettingsUpdateInput): BackendSettingsPayload {
  const payload: BackendSettingsPayload = {};

  if (input.theme) payload.theme = input.theme;
  if (input.launchAtLogin !== undefined) payload.launch_at_login = input.launchAtLogin;
  if (input.minimizeToTray !== undefined) payload.minimize_to_tray = input.minimizeToTray;
  if (input.showNotifications !== undefined) payload.show_notifications = input.showNotifications;
  if (input.notificationSound !== undefined) payload.notification_sound = input.notificationSound;
  if (input.defaultVolume !== undefined) payload.default_volume = clampVolume(input.defaultVolume);

  return payload;
}

function isThemePreference(value: string): value is ThemePreference {
  return value === 'light' || value === 'dark' || value === 'system';
}

function clampVolume(volume: number): number {
  return Math.min(100, Math.max(0, Math.round(volume)));
}
