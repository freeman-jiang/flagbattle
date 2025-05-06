import { useEffect, useRef } from 'react';

import { Input } from '@/bindings';
import { useGameStore } from '@/store';
import { encode } from '@msgpack/msgpack';

const VELOCITY_SCALE = 40;

export const Controller = () => {
    const ws = useGameStore((s) => s.ws);
    const clientId = useGameStore((s) => s.clientId);

    // Track currently-pressed movement keys
    const pressed = useRef<Set<string>>(new Set());

    // Helper to transmit velocity to server
    const sendVelocity = (dx: number, dy: number) => {
        if (!ws || ws.readyState !== WebSocket.OPEN) return;

        console.log('sendVelocity', dx, dy);
        const msg: Input = {
            playerMove: {
                playerId: clientId,
                velocity: { dx, dy }
            }
        };
        ws.send(encode(msg, { useBigInt64: true }));
    };

    // Helper to transmit melee attack to server
    const sendMeleeAttack = () => {
        if (!ws || ws.readyState !== WebSocket.OPEN) return;
        console.log('sendMeleeAttack');
        const msg: Input = {
            playerMelee: {
                player_id: clientId
            }
        };
        ws.send(encode(msg, { useBigInt64: true }));
    };

    // ---------------------------------------------------------------------
    // Keyboard listeners (WASD)
    useEffect(() => {
        const movementKeys = new Set(['w', 'a', 's', 'd'])

        const recomputeAndSend = () => {
            let dx = 0;
            let dy = 0;
            if (pressed.current.has('w')) dy -= VELOCITY_SCALE;
            if (pressed.current.has('s')) dy += VELOCITY_SCALE;
            if (pressed.current.has('a')) dx -= VELOCITY_SCALE;
            if (pressed.current.has('d')) dx += VELOCITY_SCALE;

            sendVelocity(dx, dy);
        };
        const handleDown = (e: KeyboardEvent) => {
            const key = e.key.toLowerCase();

            // Handle movement keys
            if (movementKeys.has(key)) {
                if (!pressed.current.has(key)) {
                    pressed.current.add(key);
                    recomputeAndSend();
                }
                return;
            }

            // Handle spacebar for melee attack
            if (key === ' ' && !e.repeat) {
                // Only send melee command once per keypress
                sendMeleeAttack();
            }
        };

        const handleUp = (e: KeyboardEvent) => {
            const key = e.key.toLowerCase();
            if (movementKeys.has(key)) {
                if (pressed.current.delete(key)) {
                    recomputeAndSend();
                }
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
