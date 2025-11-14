import { useEffect } from 'react';

import { useSettingsStore } from '@/stores/settingsStore';

export function useThemeSync() {
  const theme = useSettingsStore((state) => state.settings.theme);

  useEffect(() => {
    const root = document.documentElement;
    root.dataset.theme = theme;
  }, [theme]);
}
