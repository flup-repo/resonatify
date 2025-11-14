import { PropsWithChildren, useMemo, useState } from 'react';

import { Button } from '@/components/ui';
import { useSettingsStore } from '@/stores/settingsStore';

export type SettingsNavItem = 'general' | 'notifications' | 'audio' | 'about' | 'support';

interface SettingsLayoutProps {
  activeSection?: SettingsNavItem;
  onSectionChange?: (section: SettingsNavItem) => void;
}

const navItems: { id: SettingsNavItem; label: string }[] = [
  { id: 'general', label: 'General' },
  { id: 'notifications', label: 'Notifications' },
  { id: 'audio', label: 'Audio' },
  { id: 'about', label: 'About' },
  { id: 'support', label: 'Support' },
];

export function SettingsLayout({ activeSection, onSectionChange, children }: PropsWithChildren<SettingsLayoutProps>) {
  const [fallbackSection, setFallbackSection] = useState<SettingsNavItem>('general');
  const isLoading = useSettingsStore((state) => state.isLoading);
  const error = useSettingsStore((state) => state.error);
  const fetchSettings = useSettingsStore((state) => state.fetchSettings);

  const currentSection = activeSection ?? fallbackSection;

  const body = useMemo(() => {
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
      <div className="grid gap-8 lg:grid-cols-[220px_1fr]">
        <nav className="rounded-xl border bg-card p-4 text-sm">
          <ul className="space-y-1">
            {navItems.map((item) => (
              <li key={item.id}>
                <button
                  className="flex w-full items-center justify-between rounded-lg px-3 py-2 text-left text-muted-foreground transition hover:bg-muted"
                  onClick={() => {
                    onSectionChange?.(item.id);
                    setFallbackSection(item.id);
                    document.getElementById(item.id)?.scrollIntoView({ behavior: 'smooth' });
                  }}
                >
                  <span>{item.label}</span>
                  {currentSection === item.id ? <span className="text-xs">•</span> : null}
                </button>
              </li>
            ))}
          </ul>
        </nav>

        <div className="space-y-8">{children}</div>
      </div>
    );
  }, [children, currentSection, error, fetchSettings, isLoading, onSectionChange]);

  return body;
}
