import { describe, it, expect } from 'vitest';
import { cn } from '../cn';

describe('cn utility', () => {
  it('should merge class names', () => {
    expect(cn('a', 'b')).toBe('a b');
  });

  it('should handle conditional classes', () => {
    expect(cn('a', true && 'b', false && 'c')).toBe('a b');
  });

  it('should handle objects/arrays (though mostly used with strings/booleans here)', () => {
    expect(cn(['a', 'b'])).toBe('a b');
  });

  it('should merge tailwind classes correctly (override)', () => {
    expect(cn('p-4', 'p-8')).toBe('p-8');
    expect(cn('bg-red-500', 'bg-blue-500')).toBe('bg-blue-500');
  });

  it('should merge tailwind classes with different utilities', () => {
    expect(cn('px-4 py-2', 'p-8')).toBe('p-8');
  });
});
