import { create } from 'zustand';

// Technically not the actual game state, just whatever the client needs to render.
interface GameStore {
    // Networked state
    ws: WebSocket | null;
    setWS: (ws: WebSocket) => void;
}

export const useGameStore = create<GameStore>()((set) => ({
    ws: null,
    setWS: (ws: WebSocket) => set({ ws })
}));
