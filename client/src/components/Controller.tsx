import { useEffect, useRef } from 'react';

import { Input } from '@/bindings';
import { useGameStore } from '@/store';
import { encode } from '@msgpack/msgpack';

export const Controller = () => {
    const ws = useGameStore((s) => s.ws);

    // Track currently-pressed movement keys
    const pressed = useRef<Set<string>>(new Set());

    // Helper to transmit velocity to server
    const sendVelocity = (dx: number, dy: number) => {
        if (!ws || ws.readyState !== WebSocket.OPEN) return;

        console.log('sendVelocity', dx, dy);
        const msg: Input = {
            PlayerMove: {
                // FIXME: replace with actual player id once login implemented
                player_id: BigInt(1),
                velocity: { dx, dy }
            }
        };
        ws.send(encode(msg, { useBigInt64: true }));
    };

    // ---------------------------------------------------------------------
    // Keyboard listeners (WASD)
    useEffect(() => {
        const relevant = new Set(['w', 'a', 's', 'd']);

        const recomputeAndSend = () => {
            let dx = 0,
                dy = 0;
            if (pressed.current.has('w')) dy -= 1;
            if (pressed.current.has('s')) dy += 1;
            if (pressed.current.has('a')) dx -= 1;
            if (pressed.current.has('d')) dx += 1;

            sendVelocity(dx, dy);
        };

        const handleDown = (e: KeyboardEvent) => {
            const key = e.key.toLowerCase();
            if (!relevant.has(key)) return;
            if (!pressed.current.has(key)) {
                pressed.current.add(key);
                recomputeAndSend();
            }
        };

        const handleUp = (e: KeyboardEvent) => {
            const key = e.key.toLowerCase();
            if (!relevant.has(key)) return;
            if (pressed.current.delete(key)) {
                recomputeAndSend();
            }
        };

        window.addEventListener('keydown', handleDown);
        window.addEventListener('keyup', handleUp);

        return () => {
            window.removeEventListener('keydown', handleDown);
            window.removeEventListener('keyup', handleUp);
        };
    }, [ws]);

    return null;
};
