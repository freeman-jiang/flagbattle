import { useCallback, useEffect, useState } from 'react';

import { Flag, Player, Snapshot, Team } from '@/bindings';
import { useGameStore } from '@/store';
import { Application, extend } from '@pixi/react';

import { Container, Graphics, Text, TextStyle } from 'pixi.js';

extend({
    Container,
    Graphics,
    Text
});

// World dimensions (must stay in sync with Rust constants in game/src/lib.rs)
const WORLD_WIDTH = 200;
const WORLD_HEIGHT = 100;

// Utility helpers -----------------------------------------------------------
const teamColor = (team: Team) => (team === 'red' ? 0xff4d4d : 0x4d6dff);

const useViewport = () => {
    const [size, setSize] = useState({
        width: typeof window !== 'undefined' ? window.innerWidth : 800,
        height: typeof window !== 'undefined' ? window.innerHeight : 600
    });

    useEffect(() => {
        const onResize = () => setSize({ width: window.innerWidth, height: window.innerHeight });
        window.addEventListener('resize', onResize);

        return () => window.removeEventListener('resize', onResize);
    }, []);

    return size;
};

// Graphics draw callbacks ----------------------------------------------------
const createPlayerDraw = (radius: number, color: number) =>
    function draw(g: Graphics) {
        g.clear();
        g.circle(0, 0, radius).fill(color);
    };

const createFlagDraw = (size: number, color: number) =>
    function draw(g: Graphics) {
        g.clear();
        g.rect(-size / 2, -size / 2, size, size).fill(color);
    };

const createArenaDraw = (width: number, height: number) =>
    function draw(g: Graphics) {
        g.clear();
        g.rect(0, 0, width, height).stroke({ width: 1, color: 'blue', alpha: 0.4 });
    };

// Main component ------------------------------------------------------------
export const Renderer = () => {
    const snapshot: Snapshot | null = useGameStore((s) => s.snapshot);
    const { width, height } = useViewport();

    // Determine uniform scaling so the world fits in the viewport while preserving aspect ratio.
    const scale = Math.min(width / WORLD_WIDTH, height / WORLD_HEIGHT);

    // Center the arena within the viewport.
    const offsetX = (width - WORLD_WIDTH * scale) / 2;
    const offsetY = (height - WORLD_HEIGHT * scale) / 2;

    // Pre-compute draw callbacks that depend on scale so they get memoised between renders.
    const arenaDraw = useCallback(createArenaDraw(WORLD_WIDTH, WORLD_HEIGHT), []);

    // Get scores from snapshot with type assertion to handle missing type definition
    type SnapshotWithScore = Snapshot & { score?: Record<string, number> };
    const snapshotWithScore = snapshot as SnapshotWithScore;
    const redScore = snapshotWithScore?.score?.['red'] || 0;
    const blueScore = snapshotWithScore?.score?.['blue'] || 0;

    // Text style for score display
    const scoreStyle = new TextStyle({
        fontFamily: 'Arial',
        fontSize: 32,
        fontWeight: 'bold',
        fill: 'white',
        align: 'center'
    });

    return (
        <Application
            width={width}
            height={height}
            background={'black'}
            className='h-screen w-screen'
            resolution={typeof window !== 'undefined' ? window.devicePixelRatio : 1}
            autoDensity={true}
            antialias={true}>
            {/* Score display */}
            <pixiText text={`RED: ${redScore}`} x={20} y={20} style={scoreStyle} tint={0xff4d4d} resolution={2} />
            <pixiText
                text={`BLUE: ${blueScore}`}
                x={width - 120}
                y={20}
                style={scoreStyle}
                tint={0x4d6dff}
                resolution={2}
            />

            {/* Root container that shifts + scales everything to world space */}
            <pixiContainer x={offsetX} y={offsetY} scale={scale}>
                {/* Arena outline */}
                <pixiGraphics draw={arenaDraw} />

                {/* Players */}
                {snapshot?.players.map((player: Player) => {
                    const { position, metadata, team } = player;
                    const color = teamColor(team);
                    const radius = 2.5; // world units (half of rust Radius for nicer visuals)
                    const drawCb = createPlayerDraw(radius, color);

                    return <pixiGraphics key={metadata.id} x={position.x} y={position.y} draw={drawCb} />;
                })}

                {/* Flags */}
                {snapshot?.flags.map((flag: Flag, idx: number) => {
                    const posX = flag.position.x;
                    const posY = flag.position.y;
                    const size = 3; // world units
                    const color = teamColor(flag.team);
                    const drawCb = createFlagDraw(size, color);

                    return <pixiGraphics key={`flag-${flag.team}`} x={posX} y={posY} draw={drawCb} />;
                })}
            </pixiContainer>
        </Application>
    );
};
