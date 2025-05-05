import { Snapshot } from '@/bindings';

import { create } from 'zustand';

// Technically not the actual game state, just whatever the client needs to render.
interface GameStore {
    // NETWORK
    ws: WebSocket | null;
    setWS: (ws: WebSocket) => void;

    // LOCAL
    snapshot: Snapshot | null;
    setSnapshot: (snapshot: Snapshot) => void;
}

export const useGameStore = create<GameStore>()((set) => ({
    // NETWORK
    ws: null,
    setWS: (ws) => set({ ws }),

    // LOCAL
    snapshot: null,
    setSnapshot: (snapshot) => set({ snapshot })
}));
