import { useState } from 'react';

import { cn } from '@/registry/new-york-v4/lib/utils';
import { Dialog, DialogContent } from '@/registry/new-york-v4/ui/dialog';
import { useGameStore } from '@/store';

import { AlertCircleIcon } from 'lucide-react';

export const DebugPanel = () => {
    const snapshot = useGameStore((state) => state.snapshot);
    const [isVisible, setIsVisible] = useState(false);

    return (
        <>
            {/* Toggle button */}
            <button
                onClick={() => setIsVisible(!isVisible)}
                className='fixed top-4 right-4 z-10 rounded-full bg-blue-600 p-3 text-white shadow-lg transition-colors hover:bg-blue-700'>
                {isVisible ? 'Hide' : 'Debug'}
            </button>

            {/* Using DialogContent for proper keyboard handling */}
            <Dialog open={isVisible} onOpenChange={setIsVisible}>
                <DialogContent
                    className={cn(
                        'max-h-[90vh] max-w-2xl overflow-auto p-6',
                        'border border-slate-800 bg-slate-900 text-white'
                    )}
                    // Override the close button to use our preferred styling
                >
                    <h2 className='mb-6 text-xl font-semibold'>Game State</h2>

                    {!snapshot ? (
                        <div className='flex flex-col items-center justify-center py-10 text-slate-400'>
                            <AlertCircleIcon className='mb-4 h-12 w-12 text-slate-500' />
                            <p className='text-lg'>No game state available</p>
                            <p className='mt-2 text-sm'>Game data will appear here when it's loaded</p>
                        </div>
                    ) : (
                        <div className='space-y-6'>
                            <div className='rounded bg-slate-800 p-3'>
                                <h3 className='mb-3 text-lg font-medium'>Full Snapshot</h3>
                                <pre className='font-mono text-sm break-words whitespace-pre-wrap text-slate-300'>
                                    {JSON.stringify(snapshot, null, 2)}
                                </pre>
                            </div>
                        </div>
                    )}
                </DialogContent>
            </Dialog>
        </>
    );
};
