extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::{thread_rng, Rng};

pub fn gen_apple(snake: &Vec<u16>, size: u16) -> usize {
    let mut rng = thread_rng();
    let mut apple_gen: u16 = rng.gen_range(0..(900-size));

    for i in 0..900 {
        if snake[i] == 0 {
            if apple_gen == 0 {
                return i;
            }
            else {
                apple_gen -= 1;
            }
        }
    }
    return 0;
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: Vec<u16>,  //Snake position
    time: f64,
    size: u16,
    path: Vec<u16>,
    apple: usize,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 25.0);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let snake = &self.snake;
        let apple = self.apple;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for i in 0..900 {
                if snake[i] != 0
                {
                    let transform = c
                    .transform
                    .trans(x, y)
                    .trans(-447.5, -447.5)
                    .trans((((i%30) as usize)*30) as f64,(((i/30) as usize)*30) as f64);

                    // Draw a box
                    rectangle(GREEN, square, transform, gl);
                }
                else if i == apple {
                    let transform = c
                    .transform
                    .trans(x, y)
                    .trans(-447.5, -447.5)
                    .trans((((i%30) as usize)*30) as f64,(((i/30) as usize)*30) as f64);

                    rectangle(RED, square, transform, gl);
                }
            }
            
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;
        if self.time > 0.001 {
            self.time -= 0.001;
            self.path[900] = self.path[0];
            if self.path[0] as usize == self.apple {
                self.size += 1;

                //Generate new apple location
                self.apple = gen_apple(&self.snake, self.size);
            }
            else {
                for i in 0..900 {
                    if self.snake[i] > 0 {
                        self.snake[i] -= 1;
                    }
                }
            }
            self.snake[self.path[0] as usize] = self.size;
            for i in 0..900 {
                self.path[i] = self.path[i+1];
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Snake", [900, 900])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create vector of snake position
    let mut snake_1d = vec![0; 900];

    snake_1d[465] = 3;
    snake_1d[495] = 2;
    snake_1d[525] = 1;

    //Gen first apple
    let apple = gen_apple(&snake_1d, 3);

    // Create vector of snake path
    let mut snake_path = vec![0; 901];
    let mut next: u16 = 466;
    let mut top: bool = false;
    let mut middle: bool = true;

    for i in 0..900 {
        snake_path[i] = next;

        if top {
            if middle {
                if next > 30 {
                    next -= 30;
                }
                else {
                    next -= 1;
                    if next == 0 {
                        top = false;
                    }
                    else {
                        middle = false;
                    }
                }
            }
            else {
                if next < 420 {
                    next += 30;
                }
                else {
                    next -= 1;
                    middle = true;
                }
            }
        }
        else {
            if middle {
                if next < 870 {
                    next += 30;
                }
                else {
                    next += 1;
                    if next == 899 {
                        top = true;
                    }
                    else {
                        middle = false;
                    }
                }
            }
            else {
                if next > 480 {
                    next -= 30;
                }
                else {
                    next += 1;
                    middle = true;
                }
            }
        }
        
    }


    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: snake_1d,
        time: 0.0,
        size: 3,
        path: snake_path,
        apple: apple,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}