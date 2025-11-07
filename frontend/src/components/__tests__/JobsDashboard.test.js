/**
 * Unit tests for JobsDashboard.svelte
 * Tests WebSocket handling, job list rendering, and user interactions
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import JobsDashboard from '../JobsDashboard.svelte';

// Mock WebSocket
class MockWebSocket {
    constructor(url) {
        this.url = url;
        this.readyState = WebSocket.CONNECTING;
        MockWebSocket.instances.push(this);
        setTimeout(() => {
            this.readyState = WebSocket.OPEN;
            if (this.onopen) this.onopen();
        }, 10);
    }
    
    send(data) {
        this.lastSent = data;
    }
    
    close() {
        this.readyState = WebSocket.CLOSED;
        if (this.onclose) this.onclose();
    }
    
    static instances = [];
    static reset() {
        MockWebSocket.instances = [];
    }
}

global.WebSocket = MockWebSocket;

// Mock API
vi.mock('$lib/api', () => ({
    jobs: {
        listJobs: vi.fn(() => Promise.resolve({
            jobs: [
                {
                    id: 'job-1',
                    job_type: 'file_cleanup',
                    status: 'completed',
                    priority: 5,
                    created_at: '2025-11-07T10:00:00Z',
                    result: '{"files_deleted": 10}'
                },
                {
                    id: 'job-2',
                    job_type: 'thumbnail_generation',
                    status: 'running',
                    priority: 8,
                    created_at: '2025-11-07T10:05:00Z'
                }
            ],
            total: 2
        })),
        getStats: vi.fn(() => Promise.resolve({
            pending: 5,
            running: 2,
            completed: 100,
            failed: 3
        })),
        cancelJob: vi.fn(() => Promise.resolve()),
        cleanupOldJobs: vi.fn(() => Promise.resolve({ deleted: 10 }))
    },
    cron: {
        listCronJobs: vi.fn(() => Promise.resolve([
            {
                id: 'cron-1',
                name: 'Daily Cleanup',
                job_type: 'file_cleanup',
                cron_expression: '0 2 * * *',
                enabled: 1
            }
        ]))
    }
}));

describe('JobsDashboard', () => {
    beforeEach(() => {
        MockWebSocket.reset();
        vi.clearAllMocks();
    });
    
    it('renders job statistics cards', async () => {
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(screen.getByText(/pending/i)).toBeInTheDocument();
            expect(screen.getByText(/running/i)).toBeInTheDocument();
            expect(screen.getByText(/completed/i)).toBeInTheDocument();
            expect(screen.getByText(/failed/i)).toBeInTheDocument();
        });
    });
    
    it('displays job list with correct data', async () => {
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(screen.getByText('file_cleanup')).toBeInTheDocument();
            expect(screen.getByText('thumbnail_generation')).toBeInTheDocument();
            expect(screen.getByText('completed')).toBeInTheDocument();
            expect(screen.getByText('running')).toBeInTheDocument();
        });
    });
    
    it('connects to WebSocket on mount', async () => {
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(MockWebSocket.instances.length).toBe(1);
            expect(MockWebSocket.instances[0].url).toContain('/api/ws');
        });
    });
    
    it('handles WebSocket job:running event', async () => {
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(MockWebSocket.instances.length).toBe(1);
        });
        
        const ws = MockWebSocket.instances[0];
        
        // Simulate receiving job:running event
        const event = {
            kind: 'job:running',
            path: 'job-3',
            metadata: {
                job_type: 'backup_task',
                status: 'running',
                priority: 10
            }
        };
        
        ws.onmessage({ data: JSON.stringify(event) });
        
        await waitFor(() => {
            // Job list should refresh
            expect(screen.queryByText('backup_task')).toBeInTheDocument();
        });
    });
    
    it('handles WebSocket job:completed event', async () => {
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(MockWebSocket.instances.length).toBe(1);
        });
        
        const ws = MockWebSocket.instances[0];
        
        const event = {
            kind: 'job:completed',
            path: 'job-2',
            metadata: {
                job_type: 'thumbnail_generation',
                status: 'completed',
                result: { thumbnails_created: 5 }
            }
        };
        
        ws.onmessage({ data: JSON.stringify(event) });
        
        await waitFor(() => {
            // Stats should update
            expect(screen.getByText(/completed/i)).toBeInTheDocument();
        });
    });
    
    it('reconnects WebSocket on connection loss', async () => {
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(MockWebSocket.instances.length).toBe(1);
        });
        
        const firstWs = MockWebSocket.instances[0];
        firstWs.close();
        
        // Should reconnect after delay
        await waitFor(() => {
            expect(MockWebSocket.instances.length).toBe(2);
        }, { timeout: 6000 }); // Reconnection delay is 5 seconds
    });
    
    it('cancels a job when cancel button is clicked', async () => {
        const { jobs } = await import('$lib/api');
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(screen.getByText('thumbnail_generation')).toBeInTheDocument();
        });
        
        // Find and click cancel button for running job
        const cancelButtons = screen.getAllByRole('button', { name: /cancel/i });
        await fireEvent.click(cancelButtons[0]);
        
        expect(jobs.cancelJob).toHaveBeenCalledWith('job-2');
    });
    
    it('shows cron jobs table', async () => {
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(screen.getByText('Daily Cleanup')).toBeInTheDocument();
            expect(screen.getByText('0 2 * * *')).toBeInTheDocument();
        });
    });
    
    it('triggers cleanup when cleanup button is clicked', async () => {
        const { jobs } = await import('$lib/api');
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(screen.getByText(/cleanup/i)).toBeInTheDocument();
        });
        
        const cleanupButton = screen.getByRole('button', { name: /cleanup old jobs/i });
        await fireEvent.click(cleanupButton);
        
        expect(jobs.cleanupOldJobs).toHaveBeenCalledWith(7); // Default 7 days
    });
    
    it('updates stats when receiving multiple events', async () => {
        render(JobsDashboard);
        
        await waitFor(() => {
            expect(MockWebSocket.instances.length).toBe(1);
        });
        
        const ws = MockWebSocket.instances[0];
        
        // Send multiple events
        ws.onmessage({ data: JSON.stringify({
            kind: 'job:running',
            path: 'job-4',
            metadata: { status: 'running' }
        })});
        
        ws.onmessage({ data: JSON.stringify({
            kind: 'job:completed',
            path: 'job-5',
            metadata: { status: 'completed' }
        })});
        
        ws.onmessage({ data: JSON.stringify({
            kind: 'job:failed',
            path: 'job-6',
            metadata: { status: 'failed', error: 'Test error' }
        })});
        
        await waitFor(() => {
            // Stats should reflect all changes
            expect(jobs.getStats).toHaveBeenCalled();
        });
    });
});
