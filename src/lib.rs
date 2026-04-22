use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Game {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    width: f64,
    height: f64,
    bird_y: f64,
    bird_velocity: f64,
    pipes: Vec<Pipe>,
    score: i32,
    game_over: bool,
    frame_count: i32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<Game, JsValue> {
        init_panic_hook();
        
        let window = web_sys::window().expect("no global window");
        let document = window.document().expect("no document");
        let canvas = document.get_element_by_id(canvas_id)
            .expect("canvas not found")
            .dyn_into::<HtmlCanvasElement>()?;
        
        let ctx = canvas.get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        Ok(Game {
            canvas,
            ctx,
            width,
            height,
            bird_y: height / 2.0,
            bird_velocity: 0.0,
            pipes: Vec::new(),
            score: 0,
            game_over: false,
            frame_count: 0,
        })
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.bird_velocity += 0.5;
        self.bird_y += self.bird_velocity;

        if self.frame_count % 100 == 0 {
            self.pipes.push(Pipe {
                x: self.width,
                gap_top: 100.0 + (js_sys::Math::random() * 150.0),
                gap_size: 150.0,
                scored: false,
            });
        }

        for pipe in &mut self.pipes {
            pipe.x -= 3.0;
        }

        self.pipes.retain(|p| p.x > -50.0);

        for pipe in &mut self.pipes {
            if pipe.x < 60.0 && pipe.x + 40.0 > 20.0 {
                if self.bird_y < pipe.gap_top || self.bird_y + 30.0 > pipe.gap_top + pipe.gap_size {
                    self.game_over = true;
                }
            }
            if pipe.x + 40.0 < 20.0 && !pipe.scored {
                self.score += 1;
                pipe.scored = true;
            }
        }

        if self.bird_y > self.height - 30.0 || self.bird_y < 0.0 {
            self.game_over = true;
        }

        self.frame_count += 1;
    }

    pub fn draw(&self) {
        self.ctx.set_fill_style(&JsValue::from_str("#70c5ce"));
        self.ctx.fill_rect(0.0, 0.0, self.width, self.height);

        self.ctx.set_fill_style(&JsValue::from_str("#ded895"));
        self.ctx.fill_rect(0.0, self.height - 50.0, self.width, 50.0);

        self.ctx.set_fill_style(&JsValue::from_str("#e8bc2a"));
        self.ctx.begin_path();
        self.ctx.ellipse(30.0, self.bird_y + 15.0, 20.0, 15.0, 0.0, 0.0, std::f64::consts::TAU).unwrap();
        self.ctx.fill();

        self.ctx.set_fill_style(&JsValue::from_str("#fff"));
        self.ctx.begin_path();
        self.ctx.arc(38.0, self.bird_y + 12.0, 5.0, 0.0, std::f64::consts::TAU).unwrap();
        self.ctx.fill();

        self.ctx.set_fill_style(&JsValue::from_str("#000"));
        self.ctx.begin_path();
        self.ctx.arc(40.0, self.bird_y + 12.0, 2.0, 0.0, std::f64::consts::TAU).unwrap();
        self.ctx.fill();

        self.ctx.set_fill_style(&JsValue::from_str("#73bf2e"));
        for pipe in &self.pipes {
            self.ctx.fill_rect(pipe.x, 0.0, 40.0, pipe.gap_top);
            self.ctx.fill_rect(pipe.x, pipe.gap_top + pipe.gap_size, 40.0, self.height - pipe.gap_top - pipe.gap_size);
        }

        self.ctx.set_fill_style(&JsValue::from_str("#fff"));
        self.ctx.set_font("24px Arial");
        self.ctx.fill_text(&format!("Score: {}", self.score), 20.0, 40.0).unwrap();

        if self.game_over {
            self.ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.5)"));
            self.ctx.fill_rect(0.0, 0.0, self.width, self.height);
            
            self.ctx.set_fill_style(&JsValue::from_str("#fff"));
            self.ctx.set_font("48px Arial");
            self.ctx.fill_text("Game Over!", self.width / 2.0 - 120.0, self.height / 2.0).unwrap();
        }
    }

    pub fn jump(&mut self) {
        if !self.game_over {
            self.bird_velocity = -8.0;
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn reset(&mut self) {
        self.bird_y = self.height / 2.0;
        self.bird_velocity = 0.0;
        self.pipes.clear();
        self.score = 0;
        self.game_over = false;
        self.frame_count = 0;
    }
}

struct Pipe {
    x: f64,
    gap_top: f64,
    gap_size: f64,
    scored: bool,
}