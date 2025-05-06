import { Snapshot, Team } from '@/bindings';

import { create } from 'zustand';

// Technically not the actual game state, just whatever the client needs to render.
interface GameStore {
    // NETWORK
    ws: WebSocket | null;
    snapshot: Snapshot | null;
    clientId: string;
    team: Team;

    // FUNCTIONS
    setWS: (ws: WebSocket) => void;
    setSnapshot: (snapshot: Snapshot) => void;
    setClientId: (clientId: string) => void;
    setTeam: (team: Team) => void;
    reset: () => void;
}

// Generate a random string ID for client identification
const generateRandomId = () => {
    return Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
};

// Initial state definition
const initialState = {
    // NETWORK
    ws: null,
    snapshot: null,
    clientId: generateRandomId(),
    team: 'red'
} as GameStore;

export const useGameStore = create<GameStore>()((set) => ({
    ...initialState,

    // NETWORK
    setWS: (ws) => set({ ws }),
    setSnapshot: (snapshot) => set({ snapshot }),
    setClientId: (clientId) => set({ clientId }),
    setTeam: (team) => set({ team }),

    // Reset to initial state
    reset: () => set({ ...initialState, clientId: generateRandomId() })
}));
