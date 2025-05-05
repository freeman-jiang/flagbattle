import Link from 'next/link';

import { Button } from '@/ui/button';

/**
 * The main page component that renders the HomePage component.
 *
 * @returns {JSX.Element} The rendered HomePage component.
 */
const Page = () => {
    return (
        <div className='flex flex-col items-center justify-center gap-4 p-8'>
            <h1 className='text-2xl font-bold'>Start Game</h1>
            <Button asChild>
                <Link href='/game'>Start</Link>
            </Button>
        </div>
    );
};

export default Page;
