import { useEffect } from 'react';

import { Snapshot } from '@/bindings';
import { useGameStore } from '@/store';
import { decode } from '@msgpack/msgpack';

import { toast } from 'sonner';

// Generate a random string ID for client identification
const generateRandomId = () => {
    return Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
};

export const WebsocketManager = () => {
    const ws = useGameStore((state) => state.ws);
    const setWS = useGameStore((state) => state.setWS);
    const setSnapshot = useGameStore((state) => state.setSnapshot);

    useEffect(() => {
        if (!ws) {
            const clientId = generateRandomId();
            const ws = new WebSocket(`ws://localhost:8080/ws?id=${clientId}`);
            ws.binaryType = 'arraybuffer';

            ws.onopen = () => {
                console.log('WebSocket connection opened');
                toast.success('Connected to server');
            };

            ws.onmessage = async (event) => {
                try {
                    const snapshot = decode(event.data) as Snapshot;
                    console.log('Decoded snapshot', snapshot);
                    setSnapshot(snapshot);
                } catch (error) {
                    console.error('Error decoding message:', error);
                }
            };

            ws.onclose = () => {
                console.log('WebSocket connection closed');
            };

            setWS(ws);

            return;
        }
    }, [ws]);

    return null; // headless component
};
