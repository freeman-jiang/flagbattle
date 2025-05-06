import { useEffect, useRef } from 'react';
import { Input } from '@/bindings';
import { useGameStore } from '@/store';
import { encode } from '@msgpack/msgpack';

const VELOCITY_SCALE = 40;
const INPUT_SEND_INTERVAL = 100; // Send input every 100ms

export const Controller = () => {
    const ws = useGameStore((s) => s.ws);
    const clientId = useGameStore((s) => s.clientId);
    // Track currently-pressed movement keys
    const pressed = useRef<Set<string>>(new Set());

    // Helper to transmit velocity to server
    const sendVelocity = (dx: number, dy: number) => {
        if (!ws || ws.readyState !== WebSocket.OPEN) return;
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

    // Compute current velocity based on pressed keys
    const computeVelocity = () => {
        let dx = 0;
        let dy = 0;
        if (pressed.current.has('w')) dy -= VELOCITY_SCALE;
        if (pressed.current.has('s')) dy += VELOCITY_SCALE;
        if (pressed.current.has('a')) dx -= VELOCITY_SCALE;
        if (pressed.current.has('d')) dx += VELOCITY_SCALE;
        return { dx, dy };
    };

    // Send current velocity to server
    const recomputeAndSend = () => {
        const { dx, dy } = computeVelocity();
        sendVelocity(dx, dy);
    };

    // ---------------------------------------------------------------------
    // Keyboard listeners (WASD)
    useEffect(() => {
        const movementKeys = new Set(['w', 'a', 's', 'd']);

        // Reset all movement when window loses focus
        const handleBlur = () => {
            // Clear all pressed keys
            pressed.current.clear();
            // Send zero velocity to stop movement
            sendVelocity(0, 0);
        };

        // Handle visibility change (tab switching)
        const handleVisibilityChange = () => {
            if (document.visibilityState === 'hidden') {
                handleBlur();
            }
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

        // Set up periodic sending of input state
        const intervalId = setInterval(() => {
            // Only send if there are any keys pressed
            if (pressed.current.size > 0) {
                recomputeAndSend();
            }
        }, INPUT_SEND_INTERVAL);

        window.addEventListener('keydown', handleDown);
        window.addEventListener('keyup', handleUp);
        window.addEventListener('blur', handleBlur);
        document.addEventListener('visibilitychange', handleVisibilityChange);

        return () => {
            clearInterval(intervalId); // Clean up the interval
            window.removeEventListener('keydown', handleDown);
            window.removeEventListener('keyup', handleUp);
            window.removeEventListener('blur', handleBlur);
            document.removeEventListener('visibilitychange', handleVisibilityChange);
        };
    }, [ws, clientId]);

    return null;
};
