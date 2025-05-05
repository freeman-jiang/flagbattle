'use client';

import { DebugPanel } from './DebugPanel';
import { WebsocketManager } from './WebsocketManager';

export const Game = () => {
    return (
        <div>
            <WebsocketManager />
            <DebugPanel />
        </div>
    );
};
