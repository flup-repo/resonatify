import { SunMoon } from 'lucide-react';

import { cn } from '@/utils/cn';

interface ThemeIndicatorProps {
  value: string;
}

export function ThemeIndicator({ value }: ThemeIndicatorProps) {
  return (
    <div
      className={cn(
        'flex items-center gap-2 rounded-full border px-3 py-1 text-xs text-muted-foreground',
        value === 'dark' && 'bg-zinc-900 text-zinc-50',
        value === 'light' && 'bg-white text-zinc-900',
      )}
    >
      <SunMoon className="h-3.5 w-3.5" />
      <span className="capitalize">{value}</span>
    </div>
  );
}
