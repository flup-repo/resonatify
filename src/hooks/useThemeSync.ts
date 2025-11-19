import { useEffect } from 'react';

import { useSettingsStore } from '@/stores/settingsStore';

const THEME_STORAGE_KEY = 'resonatify-theme';

export function useThemeSync() {
  const theme = useSettingsStore((state) => state.settings.theme);

  // Apply theme immediately on mount from localStorage (before backend loads)
  useEffect(() => {
    const root = document.documentElement;

    // Try to get cached theme from localStorage first for instant application
    try {
      const cachedTheme = localStorage.getItem(THEME_STORAGE_KEY);
      if (cachedTheme && (cachedTheme === 'light' || cachedTheme === 'dark' || cachedTheme === 'system')) {
        root.dataset.theme = cachedTheme;
      } else if (!root.dataset.theme) {
        // Fallback to default if no cached theme
        root.dataset.theme = 'dark';
      }
    } catch (error) {
      console.error('Failed to read theme from localStorage:', error);
      if (!root.dataset.theme) {
        root.dataset.theme = 'dark';
      }
    }
  }, []);

  // Update theme when settings change and persist to localStorage
  useEffect(() => {
    const root = document.documentElement;
    root.dataset.theme = theme;

    // Cache theme in localStorage for instant application on next startup
    try {
      localStorage.setItem(THEME_STORAGE_KEY, theme);
    } catch (error) {
      console.error('Failed to save theme to localStorage:', error);
    }
  }, [theme]);
}
