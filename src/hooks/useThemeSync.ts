import { useEffect } from 'react';

import { useSettingsStore } from '@/stores/settingsStore';

export function useThemeSync() {
  const theme = useSettingsStore((state) => state.settings.theme);

  useEffect(() => {
    const root = document.documentElement;
    root.dataset.theme = theme;
    console.log('Theme set to:', theme); // Debug log
  }, [theme]);

  // Set theme immediately on mount (before settings load)
  useEffect(() => {
    const root = document.documentElement;
    if (!root.dataset.theme) {
      root.dataset.theme = theme;
      console.log('Initial theme set to:', theme); // Debug log
    }
  }, []);
}
