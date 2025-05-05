'use client';

import { GameState } from './GameState';
import { WebsocketManager } from './WebsocketManager';

export const Game = () => {
    return (
        <div>
            <WebsocketManager />
            <GameState />
        </div>
    );
};
