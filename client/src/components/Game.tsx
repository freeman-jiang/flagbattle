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

                // Check if the data is binary (ArrayBuffer)
                if (event.data instanceof ArrayBuffer) {
                    // Convert ArrayBuffer to Uint8Array for proper decoding
                    const uint8Array = new Uint8Array(event.data);
                    const data = decode(uint8Array);
                    console.log('Decoded data', data);
                } else {
                    console.error('Received non-binary message:', event.data);
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
