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

pub fn exit(snake: &Vec<u16>, path: &Vec<u16>, apple: usize, size: u16) -> bool {
    let mut exit: u16 = size+1;
    let mut room: u16 = 0;
    let mut max_index: usize = 0;
    for i in 0..900 {
        if path[i] == (apple as u16) {
            max_index = i;
            break;
        }
    }
    let mut open: Vec<u16> = Vec::new();
    let mut closed: Vec<u16> = Vec::new();
    let mut current: u16;
    let mut available: bool;

    open.push(apple as u16);

    while open.is_empty() == false && exit > room {
        current = open[0];
        closed.push(open.remove(0));

        available = true;

        for i in 0..(max_index-1) {
            if path[i] == current {
                available = false;
                break;
            }
        }

        if available && snake[current as usize] != 0 {
            available = false;
            if (max_index as u16) > (snake[current as usize]+10) {
                return true;
            }
            else if (snake[current as usize]+10-(max_index as u16)) < exit {
                exit = snake[current as usize]+10-(max_index as u16);
            }
        }

        if available {
            room += 1;
            if current < 870 {
                for i in 0..closed.len() {
                    if closed[i] == (current + 30) {
                        available = false;
                        break;
                    }
                }
                if available {
                    open.push(current+30);
                } else {
                    available = true;
                }
            }
            if current > 29 {
                for i in 0..closed.len() {
                    if closed[i] == (current-30) {
                        available = false;
                        break;
                    }
                }
                if available {
                    open.push(current-30);
                } else {
                    available = true;
                }
            }
            if current%30 != 29 {
                for i in 0..closed.len() {
                    if closed[i] == (current+1) {
                        available = false;
                        break;
                    }
                }
                if available {
                    open.push(current+1);
                } else {
                    available = true;
                }
            }
            if current%30 != 0 {
                for i in 0..closed.len() {
                    if closed[i] == (current-1) {
                        available = false;
                        break;
                    }
                }
                if available {
                    open.push(current-1);
                }
            }
        }
    }

    println!("{}, {}", exit, room);

    if exit > room {
        return false;
    }
    else {
        return true;
    }
}

pub fn astar(snake: &Vec<u16>, path: &mut Vec<u16>, apple: usize) -> bool {
    let mut open: Vec<u16> = Vec::new();
    let mut open_f: Vec<u16> = Vec::new();
    let mut open_g: Vec<u16> = Vec::new();
    let mut open_parent: Vec<u16> = Vec::new();
    let mut closed: Vec<u16> = Vec::new();
    let mut closed_g: Vec<u16> = Vec::new();
    let mut closed_parent: Vec<u16> = Vec::new();
    let mut current: usize = 0;
    let mut target: u16;
    let mut best_exit: u16 = path[898];

    for i in 1..snake.len() {
        if snake[i] > snake[current] {
            current = i;
        }
    }
    open.push(current as u16);
    open_f.push(0);
    open_g.push(0);
    open_parent.push(999);

    while open.is_empty() == false {
        current = 0;
        for i in 1..open_f.len() {
            if open_f[i] < open_f[current] {
                current = i;
            }
        }

        closed.push(open.remove(current));
        open_f.remove(current);
        closed_g.push(open_g.remove(current));
        closed_parent.push(open_parent.remove(current));

        current = closed.len()-1;

        if closed[current] == apple as u16 {
            break;
        }

        if closed[current] < 870 {
            target = closed[current] + 30;
            let mut set: bool = true;
            if set && closed.contains(&target) {
                set = false;
            }
            if set && snake[target as usize] > (closed_g[current] + 1) {
                set = false;
                if snake[target as usize] < snake[best_exit as usize] {
                    best_exit = target;
                }
            }
            if set && open.contains(&target) {
                for i in 0..open.len() {
                    if open[i] == target {
                        if open_g[i] < (closed_g[current])+2 {
                            set = false;
                        }
                        break;
                    }
                }
            }
            if set {
                open.push(target);
                if apple as u16 > target {
                    open_f.push(closed_g[current]+1+(apple as u16)-target);
                } else {
                    open_f.push(closed_g[current]+1+target-(apple as u16));
                }
                open_g.push(closed_g[current] + 1);
                open_parent.push(closed[current]);
            }
        }
        if closed[current] > 29 {
            target = closed[current] - 30;
            let mut set: bool = true;
            if set && closed.contains(&target) {
                set = false;
            }
            if set && snake[target as usize] > (closed_g[current] + 1) {
                set = false;
                if snake[target as usize] < snake[best_exit as usize] {
                    best_exit = target;
                }
            }
            if set && open.contains(&target) {
                for i in 0..open.len() {
                    if open[i] == target {
                        if open_g[i] < (closed_g[current])+2 {
                            set = false;
                        }
                        break;
                    }
                }
            }
            if set {
                open.push(target);
                if apple as u16 > target {
                    open_f.push(closed_g[current]+1+(apple as u16)-target);
                } else {
                    open_f.push(closed_g[current]+1+target-(apple as u16));
                }
                open_g.push(closed_g[current] + 1);
                open_parent.push(closed[current]);
            }
        }
        if closed[current]%30 != 29 {
            target = closed[current] + 1;
            let mut set: bool = true;
            if set && closed.contains(&target) {
                set = false;
            }
            if set && snake[target as usize] > (closed_g[current] + 1) {
                set = false;
                if snake[target as usize] < snake[best_exit as usize] {
                    best_exit = target;
                }
            }
            if set && open.contains(&target) {
                for i in 0..open.len() {
                    if open[i] == target {
                        if open_g[i] < (closed_g[current])+2 {
                            set = false;
                        }
                        break;
                    }
                }
            }
            if set {
                open.push(target);
                if apple as u16 > target {
                    open_f.push(closed_g[current]+1+(apple as u16)-target);
                } else {
                    open_f.push(closed_g[current]+1+target-(apple as u16));
                }
                open_g.push(closed_g[current] + 1);
                open_parent.push(closed[current]);
            }
        }
        if closed[current]%30 != 0 {
            target = closed[current] - 1;
            let mut set: bool = true;
            if set && closed.contains(&target) {
                set = false;
            }
            if set && snake[target as usize] > (closed_g[current] + 1) {
                set = false;
                if snake[target as usize] < snake[best_exit as usize] {
                    best_exit = target;
                }
            }
            if set && open.contains(&target) {
                for i in 0..open.len() {
                    if open[i] == target {
                        if open_g[i] < (closed_g[current])+2 {
                            set = false;
                        }
                        break;
                    }
                }
            }
            if set {
                open.push(target);
                if apple as u16 > target {
                    open_f.push(closed_g[current]+1+(apple as u16)-target);
                } else {
                    open_f.push(closed_g[current]+1+target-(apple as u16));
                }
                open_g.push(closed_g[current] + 1);
                open_parent.push(closed[current]);
            }
        }
    }

    if open.is_empty() {
        for i in 0..closed.len() {
            if closed[i] == best_exit {
                current = i;
                break;
            }
        }
        loop {
            for i in 0..900 {
                path[900-i] = path[899-i];
            }
            path[0] = closed[current];
            target = closed_parent[current];
            if target == 999 {
                break;
            }
            for i in 0..closed.len() {
                if closed[i] == target {
                    current = i;
                    break;
                }
            }
        }

        return false;
    }

    loop {
        for i in 0..900 {
            path[900-i] = path[899-i];
        }
        path[0] = closed[current];
        target = closed_parent[current];
        if target == 999 {
            break;
        }
        for i in 0..closed.len() {
            if closed[i] == target {
                current = i;
                break;
            }
        }
    }

    return true;
}

pub fn bstar(snake: &Vec<u16>, path: &mut Vec<u16>) {
    
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

        let square = rectangle::square(0.0, 0.0, 20.0);
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
                    .trans(-445.0, -445.0)
                    .trans((((i%30) as usize)*30) as f64,(((i/30) as usize)*30) as f64);

                    // Draw a box
                    rectangle(GREEN, square, transform, gl);

                    if i < 870 {
                        if snake[i]+1 == snake[i+30] {
                            rectangle(GREEN, square, transform.trans(0.0,15.0), gl);
                        }
                    }
                    if i > 29 {
                        if snake[i]+1 == snake[i-30] {
                            rectangle(GREEN, square, transform.trans(0.0,-15.0), gl);
                        }
                    }
                    if i%30 != 29 {
                        if snake[i]+1 == snake[i+1] {
                            rectangle(GREEN, square, transform.trans(15.0,0.0), gl);
                        }
                    }
                    if i%30 != 0 {
                        if snake[i]+1 == snake[i-1] {
                            rectangle(GREEN, square, transform.trans(-15.0,0.0), gl);
                        }
                    }
                }
                else if i == apple {
                    let transform = c
                    .transform
                    .trans(x, y)
                    .trans(-445.0, -445.0)
                    .trans((((i%30) as usize)*30) as f64,(((i/30) as usize)*30) as f64);

                    rectangle(RED, square, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;
        if self.time > 0.05 {
            self.time -= 0.05;
            self.path[900] = self.path[0];
            if self.path[0] as usize == self.apple {
                self.size += 1;

                self.snake[self.path[0] as usize] = self.size;

                //Generate new apple location
                self.apple = gen_apple(&self.snake, self.size);

                //Generate A* path
                let a_gen = astar(&self.snake, &mut self.path, self.apple);

                if a_gen {
                    println!("{}", exit(&self.snake, &self.path, self.apple, self.size));
                }
            }
            else {
                for i in 0..900 {
                    if self.snake[i] > 0 {
                        self.snake[i] -= 1;
                    }
                }
                self.snake[self.path[0] as usize] = self.size;
            }
            
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

    //Generate A* path
    astar(&snake_1d, &mut snake_path, apple);

    for i in 0..900 {
        snake_path[i] = snake_path[i+1];
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