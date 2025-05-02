use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, WebSocket};

// Game state
struct GameState {
    x: f64,                       // x position of the square
    y: f64,                       // y position of the square
    size: f64,                    // size of the square
    speed: f64,                   // movement speed
    keys: [bool; 4],              // WASD keys state [W, A, S, D]
    websocket: Option<WebSocket>, // WebSocket connection
}

impl GameState {
    fn new() -> Self {
        Self {
            x: 200.0,
            y: 200.0,
            size: 50.0,
            speed: 5.0,
            keys: [false, false, false, false],
            websocket: None,
        }
    }

    fn update(&mut self) {
        // Move based on key presses
        if self.keys[0] {
            // W key
            self.y -= self.speed;
        }
        if self.keys[1] {
            // A key
            self.x -= self.speed;
        }
        if self.keys[2] {
            // S key
            self.y += self.speed;
        }
        if self.keys[3] {
            // D key
            self.x += self.speed;
        }

        // Send position to server if websocket is connected
        if let Some(ws) = &self.websocket {
            if ws.ready_state() == 1 {
                // OPEN
                let message = format!("{{\"x\":{},\"y\":{}}}", self.x, self.y);
                let _ = ws.send_with_str(&message);
            }
        }
    }

    fn render(&self, ctx: &CanvasRenderingContext2d) {
        // Clear canvas
        ctx.clear_rect(0.0, 0.0, 800.0, 600.0);

        // Draw square
        ctx.set_fill_style_str("blue");
        ctx.fill_rect(self.x, self.y, self.size, self.size);
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Get canvas element
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("game-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    // Set canvas dimensions
    canvas.set_width(800);
    canvas.set_height(600);

    // Get 2D rendering context
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Create shared game state
    let game_state = Rc::new(RefCell::new(GameState::new()));

    // Set up WebSocket connection
    let ws = WebSocket::new("ws://localhost:8080/ws")?;

    // Set up WebSocket event handlers
    {
        let game_state_clone = game_state.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let message = String::from(txt);
                web_sys::console::log_1(&JsValue::from_str(&format!(
                    "Message from server: {}",
                    message
                )));
            }
        }) as Box<dyn FnMut(web_sys::MessageEvent)>);
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }

    {
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            web_sys::console::log_1(&JsValue::from_str("WebSocket connection established"));
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    {
        let onclose_callback = Closure::wrap(Box::new(move |_| {
            web_sys::console::log_1(&JsValue::from_str("WebSocket connection closed"));
        }) as Box<dyn FnMut(web_sys::CloseEvent)>);
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
    }

    // Store WebSocket in game state
    game_state.borrow_mut().websocket = Some(ws);

    // Set up keyboard event handlers
    {
        let game_state_clone = game_state.clone();
        let keydown_callback = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let mut state = game_state_clone.borrow_mut();
            match e.key().as_str() {
                "w" | "W" => state.keys[0] = true,
                "a" | "A" => state.keys[1] = true,
                "s" | "S" => state.keys[2] = true,
                "d" | "D" => state.keys[3] = true,
                _ => {}
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        document.add_event_listener_with_callback(
            "keydown",
            keydown_callback.as_ref().unchecked_ref(),
        )?;
        keydown_callback.forget();
    }

    {
        let game_state_clone = game_state.clone();
        let keyup_callback = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let mut state = game_state_clone.borrow_mut();
            match e.key().as_str() {
                "w" | "W" => state.keys[0] = false,
                "a" | "A" => state.keys[1] = false,
                "s" | "S" => state.keys[2] = false,
                "d" | "D" => state.keys[3] = false,
                _ => {}
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        document
            .add_event_listener_with_callback("keyup", keyup_callback.as_ref().unchecked_ref())?;
        keyup_callback.forget();
    }

    // Set up game loop
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // Update game state
        game_state.borrow_mut().update();

        // Render
        game_state.borrow().render(&context);

        // Schedule next frame
        web_sys::window()
            .unwrap()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    // Start the game loop
    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();

    Ok(())
}

// This is needed for better error messages in case of panics
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// For better panic messages
pub mod console_error_panic_hook {
    use std::panic;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn error(s: &str);
    }

    pub fn set_once() {
        panic::set_hook(Box::new(|info| {
            error(&format!("PANIC: {}", info));
        }));
    }
}
