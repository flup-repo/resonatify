import { Github, Heart, Volume2 } from 'lucide-react';
import { useEffect } from 'react';

import {
  Button,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Switch,
} from '@/components/ui';
import { SettingsLayout } from '@/components/settings/SettingsLayout';
import { useSettingsStore } from '@/stores/settingsStore';
import { cn } from '@/utils/cn';
import { openLink } from '@/utils/openLink';

export function SettingsPanel() {
  const {
    settings,
    isLoading,
    hasLoaded,
    error,
    fetchSettings,
    setTheme,
    toggleLaunchAtLogin,
    toggleMinimizeToTray,
    toggleNotifications,
    toggleNotificationSound,
    setDefaultVolume,
  } = useSettingsStore();

  useEffect(() => {
    if (!hasLoaded && !isLoading) {
      fetchSettings();
    }
  }, [fetchSettings, hasLoaded, isLoading]);

  const body = (() => {
    if (isLoading) {
      return (
        <div className="rounded-lg border border-dashed p-8 text-center text-sm text-muted-foreground">
          Loading settings…
        </div>
      );
    }

    if (error) {
      return (
        <div className="space-y-4 rounded-lg border border-destructive/30 bg-destructive/5 p-8 text-center">
          <p className="text-sm text-destructive">{error}</p>
          <Button size="sm" onClick={fetchSettings}>
            Retry
          </Button>
        </div>
      );
    }

    return (
      <SettingsLayout>
        <div className="space-y-8">
          <SettingsSection title="General" description="Personalize the overall experience" id="general">
            <div className="space-y-6">
              <div className="flex flex-wrap items-center justify-between gap-4">
                <div>
                  <p className="text-sm font-medium">Theme</p>
                  <p className="text-xs text-muted-foreground">Choose light, dark, or follow system</p>
                </div>
                <Select value={settings.theme} onValueChange={(value) => setTheme(value as typeof settings.theme)}>
                  <SelectTrigger className="w-40">
                    <SelectValue placeholder="Theme" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="system">System</SelectItem>
                    <SelectItem value="light">Light</SelectItem>
                    <SelectItem value="dark">Dark</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <SettingToggle
                label="Launch at login"
                description="Start Resonatify automatically"
                checked={settings.launchAtLogin}
                onCheckedChange={toggleLaunchAtLogin}
              />

              <SettingToggle
                label="Minimize to tray"
                description="Keep the app running in the background when closing"
                checked={settings.minimizeToTray}
                onCheckedChange={toggleMinimizeToTray}
              />
            </div>
          </SettingsSection>

          <SettingsSection title="Notifications" description="Manage alerts" id="notifications">
            <div className="space-y-6">
              <SettingToggle
                label="Show notifications"
                description="Display notifications when schedules run"
                checked={settings.showNotifications}
                onCheckedChange={toggleNotifications}
              />

              <SettingToggle
                label="Notification sound"
                description="Play a short chime alongside notifications"
                checked={settings.notificationSound}
                onCheckedChange={toggleNotificationSound}
                disabled={!settings.showNotifications}
              />
            </div>
          </SettingsSection>

          <SettingsSection title="Audio" description="Default playback behaviour" id="audio">
            <div className="space-y-4">
              <Label className="flex items-center gap-2 text-sm font-medium">
                <Volume2 className="h-4 w-4" /> Default volume
              </Label>
              <div className="flex flex-col gap-2">
                <input
                  type="range"
                  min={0}
                  max={100}
                  value={settings.defaultVolume}
                  onChange={(event) => setDefaultVolume(Number(event.target.value))}
                  className="accent-primary"
                />
                <span className="text-sm text-muted-foreground">{settings.defaultVolume}%</span>
              </div>
            </div>
          </SettingsSection>

          <SettingsSection title="About" description="Version details" id="about">
            <div className="space-y-3 text-sm text-muted-foreground">
              <p>Version 0.1.0</p>
              <p>Built with Tauri + React + Rust.</p>
              <Button
                variant="ghost"
                size="sm"
                className="gap-2"
                onClick={() => openLink('https://github.com/flup-repo/resonatify')}
              >
                <Github className="h-4 w-4" /> GitHub Repository
              </Button>
            </div>
          </SettingsSection>

          <SettingsSection title="Support" description="Keep the project going" id="support">
            <div className="rounded-xl border bg-muted/30 p-6 text-sm">
              <p className="text-muted-foreground">
                Resonatify is donation-supported. If it helps you stay on track, consider buying a coffee.
              </p>
              <Button className="mt-4 gap-2" onClick={() => openLink('https://buymeacoffee.com/resonatify')}>
                <Heart className="h-4 w-4" /> Donate
              </Button>
            </div>
          </SettingsSection>
        </div>
      </SettingsLayout>
    );
  })();

  return body;
}

interface SettingToggleProps {
  label: string;
  description?: string;
  checked: boolean;
  onCheckedChange: (value: boolean) => void;
  disabled?: boolean;
}

function SettingToggle({ label, description, checked, onCheckedChange, disabled }: SettingToggleProps) {
  return (
    <div
      className={cn('flex flex-wrap items-center gap-4 rounded-xl border bg-card/60 p-4', disabled && 'opacity-60')}
    >
      <div className="flex-1 min-w-[200px]">
        <p className="text-sm font-medium">{label}</p>
        {description ? <p className="text-xs text-muted-foreground">{description}</p> : null}
      </div>
      <Switch checked={checked} onCheckedChange={onCheckedChange} disabled={disabled} />
    </div>
  );
}

interface SettingsSectionProps {
  title: string;
  description: string;
  id: string;
  children: React.ReactNode;
}

function SettingsSection({ title, description, id, children }: SettingsSectionProps) {
  return (
    <section id={id} className="space-y-4">
      <div>
        <h2 className="text-xl font-semibold tracking-tight">{title}</h2>
        <p className="text-sm text-muted-foreground">{description}</p>
      </div>
      <div className="space-y-4 rounded-2xl border bg-card/70 p-6 shadow-sm">{children}</div>
    </section>
  );
}
