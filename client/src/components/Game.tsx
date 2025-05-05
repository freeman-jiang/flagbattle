'use client';

import { useEffect } from 'react';

import { useGameStore } from '@/store';
import { decode } from '@msgpack/msgpack';

export const Game = () => {
    const { ws } = useGameStore();

    useEffect(() => {
        if (!ws) {
            const ws = new WebSocket('ws://localhost:8080/ws');

            ws.onopen = () => {
                console.log('WebSocket connection opened');
            };

            ws.onmessage = (event) => {
                console.log('WebSocket message received', event.data);
                const data = decode(event.data);
                console.log('Decoded data', data);
            };

            ws.onclose = () => {
                console.log('WebSocket connection closed');
            };

            return;
        }
    }, [ws]);

    return <div>Game</div>;
};
