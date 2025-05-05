import { useEffect } from 'react';

import { Snapshot } from '@/bindings';
import { useGameStore } from '@/store';
import { decode } from '@msgpack/msgpack';

import { toast } from 'sonner';

export const WebsocketManager = () => {
    const ws = useGameStore((state) => state.ws);
    const setWS = useGameStore((state) => state.setWS);
    const setSnapshot = useGameStore((state) => state.setSnapshot);
    const clientId = useGameStore((state) => state.clientId);

    useEffect(() => {
        if (!ws) {
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
