import { useState } from 'react';

import { cn } from '@/registry/new-york-v4/lib/utils';
// Importing dialog components but we'll create a custom implementation
import { Dialog, DialogOverlay } from '@/registry/new-york-v4/ui/dialog';
import { useGameStore } from '@/store';

import { AlertCircleIcon, XIcon } from 'lucide-react';

export const GameState = () => {
    const snapshot = useGameStore((state) => state.snapshot);
    const [isVisible, setIsVisible] = useState(false);

    return (
        <>
            {/* Toggle button */}
            <button
                onClick={() => setIsVisible(!isVisible)}
                className='fixed right-4 bottom-4 z-10 rounded-full bg-blue-600 p-3 text-white shadow-lg transition-colors hover:bg-blue-700'>
                {isVisible ? 'Hide' : 'Debug'}
            </button>

            {/* Custom Dialog implementation for more control */}
            {isVisible && (
                <Dialog open={isVisible} onOpenChange={setIsVisible}>
                    <DialogOverlay className='bg-black/80' />
                    <div
                        className='fixed inset-0 z-50 flex items-center justify-center'
                        onClick={() => setIsVisible(false)}>
                        <div
                            className={cn(
                                'relative max-h-[90vh] w-[90vw] max-w-2xl overflow-auto rounded-lg p-6 shadow-lg',
                                'border border-slate-800 bg-slate-900 text-white'
                            )}
                            onClick={(e) => e.stopPropagation()}>
                            <button
                                onClick={() => setIsVisible(false)}
                                className='absolute top-3 right-3 rounded-sm text-white opacity-70 transition-opacity hover:opacity-100'
                                aria-label='Close'>
                                <XIcon className='h-4 w-4' />
                            </button>

                            <h2 className='mb-6 text-xl font-semibold'>Game State</h2>

                            {!snapshot ? (
                                <div className='flex flex-col items-center justify-center py-10 text-slate-400'>
                                    <AlertCircleIcon className='mb-4 h-12 w-12 text-slate-500' />
                                    <p className='text-lg'>No game state available</p>
                                    <p className='mt-2 text-sm'>Game data will appear here when it's loaded</p>
                                </div>
                            ) : (
                                <div className='space-y-6'>
                                    <div>
                                        <h3 className='mb-3 text-lg font-medium'>
                                            Players ({snapshot.players.length})
                                        </h3>
                                        <div className='space-y-2'>
                                            {snapshot.players.length === 0 ? (
                                                <div className='text-slate-400'>No players</div>
                                            ) : (
                                                snapshot.players.map((player, index) => (
                                                    <div key={index} className='rounded bg-slate-800 p-3'>
                                                        <pre className='font-mono text-sm break-words whitespace-pre-wrap text-slate-300'>
                                                            {JSON.stringify(player, null, 2)}
                                                        </pre>
                                                    </div>
                                                ))
                                            )}
                                        </div>
                                    </div>

                                    <div>
                                        <h3 className='mb-3 text-lg font-medium'>Flags ({snapshot.flags.length})</h3>
                                        <div className='space-y-2'>
                                            {snapshot.flags.map((flag, index) => (
                                                <div key={index} className='rounded bg-slate-800 p-3'>
                                                    <pre className='font-mono text-sm break-words whitespace-pre-wrap text-slate-300'>
                                                        {JSON.stringify(flag, null, 2)}
                                                    </pre>
                                                </div>
                                            ))}
                                        </div>
                                    </div>
                                </div>
                            )}
                        </div>
                    </div>
                </Dialog>
            )}
        </>
    );
};
