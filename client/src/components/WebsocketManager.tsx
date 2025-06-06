import { useEffect } from 'react';

import { Snapshot } from '@/bindings';
import { useGameStore } from '@/store';
import { decode } from '@msgpack/msgpack';

import { toast } from 'sonner';

const WS_URL = process.env.NEXT_PUBLIC_WS_URL;
if (!WS_URL) {
    throw new Error('NEXT_PUBLIC_WS_URL is not set');
}

export const WebsocketManager = () => {
    const ws = useGameStore((state) => state.ws);
    const setWS = useGameStore((state) => state.setWS);
    const setSnapshot = useGameStore((state) => state.setSnapshot);
    const clientId = useGameStore((state) => state.clientId);
    const team = useGameStore((state) => state.team);
    useEffect(() => {
        if (!ws) {
            const ws = new WebSocket(`${WS_URL}/ws?id=${clientId}&team=${team}`);
            ws.binaryType = 'arraybuffer';

            ws.onopen = () => {
                console.log('WebSocket connection opened');
                toast.success('Connected to server');
            };

            ws.onmessage = async (event) => {
                try {
                    const snapshot = decode(event.data) as Snapshot;
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
