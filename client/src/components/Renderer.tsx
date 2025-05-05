import { useCallback } from 'react';

import { Application, extend } from '@pixi/react';

import { Container, Graphics } from 'pixi.js';

extend({
    Container,
    Graphics
});

export const Renderer = () => {
    const drawCallback = useCallback((graphics: Graphics) => {
        graphics.clear();
        graphics.setFillStyle({ color: 'red' });
        graphics.rect(0, 0, 100, 100);
        graphics.fill();
    }, []);

    return (
        <Application className='h-screen w-screen'>
            <pixiContainer x={100} y={100}>
                <pixiGraphics draw={drawCallback} />
            </pixiContainer>
        </Application>
    );
};
