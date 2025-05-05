import { Snapshot, Team } from '@/bindings';

import { create } from 'zustand';

// Technically not the actual game state, just whatever the client needs to render.
interface GameStore {
    // NETWORK
    ws: WebSocket | null;
    setWS: (ws: WebSocket) => void;

    // LOCAL
    snapshot: Snapshot | null;
    setSnapshot: (snapshot: Snapshot) => void;

    clientId: string;
    setClientId: (clientId: string) => void;

    team: Team;
    setTeam: (team: Team) => void;
}

// Generate a random string ID for client identification
const generateRandomId = () => {
    return Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
};

export const useGameStore = create<GameStore>()((set) => ({
    // NETWORK
    ws: null,
    setWS: (ws) => set({ ws }),

    // LOCAL
    snapshot: null,
    setSnapshot: (snapshot) => set({ snapshot }),
    clientId: generateRandomId(),
    setClientId: (clientId) => set({ clientId }),
    team: 'red',
    setTeam: (team) => set({ team })
}));
