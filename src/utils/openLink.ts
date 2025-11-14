import { invoke } from '@tauri-apps/api/core';

export async function openLink(url: string) {
  try {
    await invoke('open', { url });
  } catch (error) {
    console.error('Failed to open link', error);
  }
}
