export type RepeatKind = 'once' | 'daily' | 'weekdays' | 'weekends' | 'custom' | 'weekly';

export type RepeatType =
  | { type: 'once' }
  | { type: 'daily' }
  | { type: 'weekdays' }
  | { type: 'weekends' }
  | { type: 'custom'; intervalMinutes: number }
  | { type: 'weekly'; days: number[] }; // 0-6 (Sun-Sat, Sunday = 0)

export type BackendRepeatType =
  | { type: 'once' }
  | { type: 'daily' }
  | { type: 'weekdays' }
  | { type: 'weekends' }
  | { type: 'custom'; interval_minutes: number }
  | { type: 'weekly'; days: BackendWeekday[] };

type BackendWeekday = string | number;

export const WEEKDAY_LABELS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

const WEEKDAY_VALUES = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
const WEEKDAY_TO_INDEX = WEEKDAY_VALUES.reduce<Record<string, number>>((acc, label, index) => {
  acc[label.toLowerCase()] = index;
  return acc;
}, {});

function normalizeBackendDay(value: BackendWeekday): number | undefined {
  if (typeof value === 'number') {
    return value % 7;
  }

  const key = value.toLowerCase();
  if (WEEKDAY_TO_INDEX[key] !== undefined) {
    return WEEKDAY_TO_INDEX[key];
  }

  return undefined;
}

export function formatRepeatType(repeat: RepeatType): string {
  switch (repeat.type) {
    case 'once':
      return 'Once';
    case 'daily':
      return 'Daily';
    case 'weekdays':
      return 'Weekdays';
    case 'weekends':
      return 'Weekends';
    case 'custom':
      return `Every ${repeat.intervalMinutes}m`;
    case 'weekly':
      return repeat.days && repeat.days.length
        ? repeat.days.map((d) => WEEKDAY_LABELS[d]).join(', ')
        : 'Weekly';
    default:
      return 'Custom';
  }
}

export function toBackendRepeatType(repeat: RepeatType): BackendRepeatType {
  switch (repeat.type) {
    case 'custom':
      return { type: 'custom', interval_minutes: repeat.intervalMinutes };
    case 'weekly':
      return { type: 'weekly', days: (repeat.days ?? []).map((index) => WEEKDAY_VALUES[index] ?? 'Sun') };
    default:
      return repeat;
  }
}

export function fromBackendRepeatType(repeat: BackendRepeatType): RepeatType {
  switch (repeat.type) {
    case 'once':
    case 'daily':
    case 'weekdays':
    case 'weekends':
      return repeat;
    case 'custom':
      return { type: 'custom', intervalMinutes: repeat.interval_minutes };
    case 'weekly': {
      const normalized = (repeat.days ?? [])
        .map((value) => normalizeBackendDay(value))
        .filter((value): value is number => value !== undefined)
        .map((value) => value % 7);
      return { type: 'weekly', days: normalized };
    }
    default:
      return { type: 'once' };
  }
}
