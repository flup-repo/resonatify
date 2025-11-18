import { test, expect } from '@playwright/test';

test.describe('Schedule Management', () => {
  test.beforeEach(async ({ page }) => {
    // Mock Tauri IPC
    await page.addInitScript(() => {
      // Setup mock store
      const schedules = [];
      const settings = {
        theme: 'system',
        launch_at_login: false,
        minimize_to_tray: false,
        show_notifications: true,
        notification_sound: true,
        default_volume: 100,
        announcement_enabled: false,
        announcement_sound: 'spell'
      };

      // Mock the internal invoke function
      // Tauri v2 uses window.__TAURI_INTERNALS__.invoke
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args: any) => {
          console.log(`[MockIPC] ${cmd}`, args);
          
          switch (cmd) {
            case 'get_all_schedules':
              return schedules;
            
            case 'get_settings':
              return settings;
              
            case 'get_scheduler_status':
              return {
                is_running: false,
                total_schedules: schedules.length,
                schedules: []
              };
              
            case 'get_upcoming_executions':
              return [];
              
            case 'create_schedule':
              const newSchedule = {
                id: 'test-id-' + Date.now(),
                ...args.input,
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString(),
                last_run_at: null
              };
              schedules.push(newSchedule);
              return newSchedule;
              
            case 'open_audio_file_dialog':
              return '/mock/path/to/audio.mp3';
              
            case 'validate_audio_file':
              return {
                valid: true,
                duration_ms: 60000,
                format: 'mp3'
              };

            case 'start_scheduler':
            case 'stop_scheduler':
            case 'reload_scheduler':
            case 'plugin:opener|open_url':
              return null;

            default:
              console.warn(`[MockIPC] Unhandled command: ${cmd}`);
              return null;
          }
        }
      };
    });
  });

  test('should display empty state initially', async ({ page }) => {
    await page.goto('/');
    
    // Expect empty state message
    await expect(page.getByText('No schedules yet')).toBeVisible();
    await expect(page.getByText('Create your first schedule')).toBeVisible();
  });

  test('should open modal and create a schedule', async ({ page }) => {
    await page.goto('/');
    
    // Click "New Schedule" button in the header
    await page.getByRole('button', { name: 'New Schedule' }).click();
    
    // Check modal title
    await expect(page.getByRole('dialog')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Create schedule' })).toBeVisible();
    
    // Fill form
    await page.getByLabel('Name').fill('Morning Alarm');
    
    // Test file picker interaction
    await page.getByRole('button', { name: 'Browse' }).click();
    // The mock returns '/mock/path/to/audio.mp3', which should appear in the input
    await expect(page.getByLabel('Audio file')).toHaveValue('/mock/path/to/audio.mp3');
    
    // Set time
    await page.getByLabel('Time').fill('08:00');
    
    // Submit
    await page.getByRole('button', { name: 'Create' }).click();
    
    // Modal should close
    await expect(page.getByRole('dialog')).toBeHidden();
    
    // New schedule should be visible in the list
    await expect(page.getByText('Morning Alarm')).toBeVisible();
    // Time is formatted (e.g. 08:00 -> 8:00 AM)
    await expect(page.getByText('8:00 AM')).toBeVisible();
  });
});
