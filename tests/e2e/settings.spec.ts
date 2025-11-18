import { test, expect } from '@playwright/test';

test.describe('Settings Management', () => {
  test.beforeEach(async ({ page }) => {
    // Mock Tauri IPC
    await page.addInitScript(() => {
      let settings = {
        theme: 'system',
        launch_at_login: false,
        minimize_to_tray: false,
        show_notifications: true,
        notification_sound: true,
        default_volume: 80,
        announcement_enabled: false,
        announcement_sound: 'spell'
      };

      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args: any) => {
          console.log(`[MockIPC] ${cmd}`, args);
          
          switch (cmd) {
            case 'get_settings':
              return settings;
              
            case 'update_settings':
              // The frontend sends { payload: { ...partial } }
              // We need to merge it with current settings and return the FULL object
              const updates = args.payload || {};
              // Backend uses snake_case
              settings = {
                 ...settings,
                 ...updates
              };
              // But args.payload is snake_case. e.g. launch_at_login
              // My mock settings object is snake_case (from get_settings return).
              // Let's ensure we handle it correctly.
              // The code: `toBackendPayload` produces snake_case.
              // So args.payload has snake_case keys.
              // So spreading ...args.payload into settings is correct.
              
              return settings;

            case 'set_launch_at_login':
              settings.launch_at_login = args.enabled;
              return null;

            case 'plugin:opener|open_url':
              return null;

            default:
              return null;
          }
        }
      };
    });
  });

  test('should load and display settings', async ({ page }) => {
    await page.goto('/');
    
    // Switch to Settings tab
    await page.getByRole('button', { name: 'Settings' }).click();
    
    // Check sidebar exists (Nav)
    await expect(page.locator('nav ul')).toBeVisible();
    
    // Check General section is visible
    await expect(page.getByRole('heading', { name: 'General' })).toBeVisible();
    
    // Check General tab content
    await expect(page.getByLabel('Launch at login')).not.toBeChecked();
    await expect(page.getByLabel('Minimize to tray')).not.toBeChecked();
  });

  test('should update settings', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Settings' }).click();
    
    // Toggle Launch at login
    const launchSwitch = page.getByRole('switch', { name: 'Launch at login' });
    await launchSwitch.click();
    await expect(launchSwitch).toBeChecked();
    
    // Change theme to Dark
    // Opens the Select
    await page.getByRole('combobox').click();
    await page.getByRole('option', { name: 'Dark' }).click();
  });

  test('should navigate between sections', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Settings' }).click();
    
    // Default shows General
    await expect(page.getByRole('heading', { name: 'General' })).toBeVisible();
    
    // Click Notifications in sidebar
    // Note: Sidebar buttons are just text buttons in a list
    await page.getByRole('button', { name: 'Notifications' }).click();
    
    // Check if Notifications section is present (it's always present, but we can check visibility)
    await expect(page.getByRole('heading', { name: 'Notifications' })).toBeVisible();
    await expect(page.getByLabel('Show notifications')).toBeChecked();
    
    // Click Audio in sidebar
    await page.getByRole('button', { name: 'Audio' }).click();
    await expect(page.getByRole('heading', { name: 'Audio' })).toBeVisible();
    await expect(page.getByText('Default volume')).toBeVisible();
  });
});
