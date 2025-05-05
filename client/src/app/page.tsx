'use client';

import Link from 'next/link';

import { cn } from '@/lib/utils';
import { useGameStore } from '@/store';
import { Button } from '@/ui/button';

/**
 * The main page component that renders the HomePage component.
 *
 * @returns {JSX.Element} The rendered HomePage component.
 */
const Page = () => {
    const { team, setTeam } = useGameStore();

    return (
        <div className='flex flex-col items-center justify-center gap-4 p-8'>
            <h1 className='text-2xl font-bold'>Flag Battle</h1>

            <div className='w-full max-w-xs space-y-2'>
                <p className='text-center text-sm font-medium'>Choose your team:</p>
                <div className='flex justify-center gap-4'>
                    <Button
                        variant={team === 'red' ? 'default' : 'outline'}
                        className={cn(
                            'w-24 flex-col',
                            team === 'red' && 'ring-2 ring-red-500',
                            team === 'red' ? 'bg-red-500 hover:bg-red-600' : 'hover:border-red-500'
                        )}
                        onClick={() => setTeam('red')}>
                        <div className='mb-1 size-6 rounded-full border border-white bg-red-500' />
                        Red Team
                    </Button>

                    <Button
                        variant={team === 'blue' ? 'default' : 'outline'}
                        className={cn(
                            'w-24 flex-col',
                            team === 'blue' && 'ring-2 ring-blue-500',
                            team === 'blue' ? 'bg-blue-500 hover:bg-blue-600' : 'hover:border-blue-500'
                        )}
                        onClick={() => setTeam('blue')}>
                        <div className='mb-1 size-6 rounded-full border border-white bg-blue-500' />
                        Blue Team
                    </Button>
                </div>
            </div>

            <Button asChild className='mt-6'>
                <Link href='/game'>Start Game</Link>
            </Button>
        </div>
    );
};

export default Page;
