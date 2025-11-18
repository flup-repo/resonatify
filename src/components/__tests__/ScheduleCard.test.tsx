import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { ScheduleCard } from '../ScheduleCard';
import { Schedule } from '@/types/schedule';

const mockSchedule: Schedule = {
  id: '1',
  name: 'Test Schedule',
  audioFilePath: '/path/to/audio.mp3',
  scheduledTime: '14:30',
  enabled: true,
  repeatType: { type: 'daily' },
  volume: 80,
  createdAt: '2023-01-01T00:00:00Z',
  updatedAt: '2023-01-01T00:00:00Z',
};

describe('ScheduleCard', () => {
  const onToggle = vi.fn();
  const onEdit = vi.fn();
  const onDelete = vi.fn();
  const onTest = vi.fn();
  const onStopTest = vi.fn();

  it('renders schedule details correctly', () => {
    render(
      <ScheduleCard
        schedule={mockSchedule}
        onToggle={onToggle}
        onEdit={onEdit}
        onDelete={onDelete}
        onTest={onTest}
        onStopTest={onStopTest}
        isTesting={false}
        isTestLoading={false}
      />
    );

    expect(screen.getByText('Test Schedule')).toBeInTheDocument();
    // "Daily at 2:30 PM • Volume 80%"
    expect(screen.getByText(/Daily at 2:30 PM/)).toBeInTheDocument();
    expect(screen.getByText(/Volume 80%/)).toBeInTheDocument();
  });

  it('calls handlers on button clicks', () => {
    render(
      <ScheduleCard
        schedule={mockSchedule}
        onToggle={onToggle}
        onEdit={onEdit}
        onDelete={onDelete}
        onTest={onTest}
        onStopTest={onStopTest}
        isTesting={false}
        isTestLoading={false}
      />
    );

    fireEvent.click(screen.getByText('Test'));
    expect(onTest).toHaveBeenCalledWith(mockSchedule);

    fireEvent.click(screen.getByText('Edit'));
    expect(onEdit).toHaveBeenCalledWith(mockSchedule);

    fireEvent.click(screen.getByText('Delete'));
    expect(onDelete).toHaveBeenCalledWith('1');
  });

  it('toggles switch', () => {
    render(
      <ScheduleCard
        schedule={mockSchedule}
        onToggle={onToggle}
        onEdit={onEdit}
        onDelete={onDelete}
        onTest={onTest}
        onStopTest={onStopTest}
        isTesting={false}
        isTestLoading={false}
      />
    );

    const switchEl = screen.getByRole('switch', { name: /toggle schedule/i });
    fireEvent.click(switchEl);
    expect(onToggle).toHaveBeenCalledWith('1', false); // Was true, toggles to false
  });

  it('shows testing state', () => {
    render(
      <ScheduleCard
        schedule={mockSchedule}
        onToggle={onToggle}
        onEdit={onEdit}
        onDelete={onDelete}
        onTest={onTest}
        onStopTest={onStopTest}
        isTesting={true}
        isTestLoading={false}
      />
    );

    expect(screen.getByText('Stop')).toBeInTheDocument();
    fireEvent.click(screen.getByText('Stop'));
    expect(onStopTest).toHaveBeenCalled();
  });
});
