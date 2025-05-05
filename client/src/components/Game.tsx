'use client';

import { DebugPanel } from './DebugPanel';
import { Renderer } from './Renderer';
import { WebsocketManager } from './WebsocketManager';

export const Game = () => {
    return (
        <div>
            <WebsocketManager />
            <Renderer />
            <DebugPanel />
        </div>
    );
};
