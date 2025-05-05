'use client';

import { useEffect } from 'react';

import { Snapshot } from '@/bindings';
import { useGameStore } from '@/store';
import { decode } from '@msgpack/msgpack';

export const Game = () => {
    const { ws } = useGameStore();

    useEffect(() => {
        if (!ws) {
            const ws = new WebSocket('ws://localhost:8080/ws');
            ws.binaryType = 'arraybuffer';

            ws.onopen = () => {
                console.log('WebSocket connection opened');
            };

            ws.onmessage = async (event) => {
                console.log('WebSocket message received', event.data);

                try {
                    const snapshot = decode(event.data) as Snapshot;
                    console.log('Decoded data', snapshot);
                } catch (error) {
                    console.error('Error decoding message:', error);
                }
            };

            ws.onclose = () => {
                console.log('WebSocket connection closed');
            };

            return;
        }
    }, [ws]);

    return <div>Game</div>;
};
