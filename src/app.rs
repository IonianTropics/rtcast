use glam::{Mat2, Vec2, Vec3};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

// DEBUG CONST

const DEBUG_TOPDOWN: bool = false;

// Level data
const LEVEL_WIDTH: usize = 32;
const LEVEL_HEIGHT: usize = 24;
const LEVEL: [[usize; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

// Colors
const TOPDOWN_LEVEL_COLOR: Vec3 = Vec3::splat(0.8);
const TOPDOWN_GRID_COLOR: Vec3 = Vec3::splat(0.3);
const TOPDOWN_WALL_COLOR: Vec3 = Vec3::splat(0.5);
const TOPDOWN_RAY_COLOR: Vec3 = Vec3::new(0.97, 0.83, 0.4);
const TOPDOWN_PLAYER_COLOR: Vec3 = Vec3::X;

const NS_WALL_COLOR: Vec3 = Vec3::new(0.0, 1.0, 1.0);
const EW_WALL_COLOR: Vec3 = Vec3::new(1.0,0.0, 1.0);
const FLOOR_COLOR: Vec3 = Vec3::new(0.9, 0.1, 0.0);
// Meta
const COLOR_FADEAWAY: f32 = 3.0;

// Window data
const TITLE: &str = "Real Time Raycasting";

// Screen data
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 120;

// Color data
const COLOR_DEPTH: usize = 4;
const COLOR_MAXVAL: u8 = 15;
const MAXVAL_MAX: u8 = 255; // DO NOT CHANGE

// Immutable Player Data
const SPEED: f32 = 0.03;
const ROTATE_SPEED: f32 = 0.03;
const RADIUS_SQUARED: f32 = 25.0;
const WALL_PADDING: f32 = 0.3;
const INITIAL_POSITION: Vec2 = Vec2::new(3.0, 3.0);
const INITIAL_LOOK: Vec2 = Vec2::new(1.0, 0.0);
const INITIAL_VIEWPORT: Vec2 = Vec2::new(0.0, 0.6);

// Camera Data
// const FOV: f32 = PI / 3.0;
const Z_FAR: f32 = 7.0;

#[derive(Debug)]
pub struct App {
    window: Option<Window>,
    pixels: Option<Pixels>,
    position: Vec2,
    look: Vec2,
    viewport: Vec2,
    key_a: ElementState,
    key_d: ElementState,
    key_s: ElementState,
    key_w: ElementState,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    fn update(&mut self) {
        let mut velocity_sign = 0.0;

        if self.key_s.is_pressed() {
            velocity_sign -= 1.0;
        }
        if self.key_w.is_pressed() {
            velocity_sign += 1.0;
        }

        let mut rot_sign = 0.0;

        if self.key_a.is_pressed() {
            rot_sign -= 1.0;
        }
        if self.key_d.is_pressed() {
            rot_sign += 1.0;
        }

        let rot = Mat2::from_angle(rot_sign * ROTATE_SPEED);
        self.look = rot * self.look;
        self.viewport = rot * self.viewport;

        let delta = velocity_sign * self.look * SPEED;
        let sign = delta.signum();
        let new_position = self.position + delta;
        let buffer = new_position + sign * WALL_PADDING;

        if LEVEL[buffer.y as usize][self.position.x as usize] == 0 {
            self.position.y = new_position.y;
        }

        if LEVEL[self.position.y as usize][buffer.x as usize] == 0 {
            self.position.x = new_position.x;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            position: INITIAL_POSITION,
            look: INITIAL_LOOK,
            viewport: INITIAL_VIEWPORT,
            key_a: ElementState::Released,
            key_d: ElementState::Released,
            key_s: ElementState::Released,
            key_w: ElementState::Released,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title(TITLE)
            .with_inner_size(PhysicalSize::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
        let window = self.window.as_ref().unwrap();
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        self.pixels =
            Some(Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(physical_size) => {
                self.pixels
                    .as_mut()
                    .unwrap()
                    .resize_surface(physical_size.width, physical_size.height)
                    .unwrap();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        repeat: false,
                        ..
                    },
                ..
            } => match key_code {
                KeyCode::KeyA => {
                    self.key_a = state;
                }
                KeyCode::KeyD => {
                    self.key_d = state;
                }
                KeyCode::KeyS => {
                    self.key_s = state;
                }
                KeyCode::KeyW => {
                    self.key_w = state;
                }
                _ => (),
            },
            WindowEvent::RedrawRequested => {
                self.update();

                // Render

                let frame = self.pixels.as_mut().unwrap().frame_mut();

                let scale = Vec2::new(
                    SCREEN_WIDTH as f32 / LEVEL_WIDTH as f32,
                    SCREEN_HEIGHT as f32 / LEVEL_HEIGHT as f32,
                );
                let screen_position = scale * self.position;

                if DEBUG_TOPDOWN {
                    draw_scene_topdown(frame);
                }

                // Raycasting

                for i in 0..SCREEN_WIDTH {
                    let ray_direction = self.look + self.viewport * (2.0 * i as f32 / SCREEN_WIDTH as f32 - 1.0);

                    let mut map_index = self.position.as_ivec2();
                    let step = ray_direction.signum().as_ivec2();

                    let t_delta = ray_direction.recip().abs();

                    let mut t_max = Vec2::new(
                        if step.x > 0 {
                            (map_index.x as f32 + 1.0 - self.position.x) * t_delta.x
                        } else {
                            (self.position.x - map_index.x as f32) * t_delta.x
                        },
                        if step.y > 0 {
                            (map_index.y as f32 + 1.0 - self.position.y) * t_delta.y
                        } else {
                            (self.position.y - map_index.y as f32) * t_delta.y
                        },
                    );

                    let mut side;

                    loop {
                        if t_max.x < t_max.y {
                            t_max.x += t_delta.x;
                            map_index.x += step.x;
                            side = 0;
                        } else {
                            t_max.y += t_delta.y;
                            map_index.y += step.y;
                            side = 1;
                        }
                        if LEVEL[map_index.y as usize][map_index.x as usize] > 0 {
                            break;
                        }
                    }

                    let orthographic_distance = if side == 0 {
                        t_max.x - t_delta.x
                    } else {
                        t_max.y - t_delta.y
                    };

                    let projection_distance = ray_direction.length() * orthographic_distance;

                    // Draw rays else 3d scene
                    if DEBUG_TOPDOWN {
                        draw_rays(self.position, scale, ray_direction, projection_distance, frame);
                    } else {
                        draw_column_first_person(projection_distance, frame, i, side);
                    }
                }

                draw_player(screen_position, frame);

                self.pixels.as_ref().unwrap().render().unwrap();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn draw_scene_topdown(frame: &mut [u8]) {
    for j in 0..SCREEN_HEIGHT {
        for i in 0..SCREEN_WIDTH {

            let y = LEVEL_HEIGHT * j / SCREEN_HEIGHT;
            let x = LEVEL_WIDTH * i / SCREEN_WIDTH;

            let edge_x = SCREEN_WIDTH.checked_div(LEVEL_WIDTH).unwrap();
            let edge_y = SCREEN_HEIGHT.checked_div(LEVEL_HEIGHT).unwrap();

            let color = if LEVEL[y][x] == 1 {
                TOPDOWN_WALL_COLOR
            } else if i % edge_x < 1 || j % edge_y < 1 {
                TOPDOWN_GRID_COLOR
            } else {
                TOPDOWN_LEVEL_COLOR
            };

            draw(frame, i, j, color);
        }
    }
}

fn draw_rays(position: Vec2, scale: Vec2, ray_direction: Vec2, projection_distance: f32, frame: &mut [u8]) {
    let start = (scale * position).as_ivec2();
    let end = (scale * (position + ray_direction.normalize() * projection_distance)).as_ivec2();

    let delta_x = (end.x - start.x).abs();
    let delta_y = -(end.y - start.y).abs();

    let sign_x = if start.x < end.x {
        1
    } else {
        -1
    };
    let sign_y = if start.y < end.y {
        1
    } else {
        -1
    };

    let mut error = delta_x + delta_y;

    let mut x = start.x;
    let mut y = start.y;

    loop {
        draw(frame, x as usize, y as usize, TOPDOWN_RAY_COLOR);
        if x == end.x && y == end.y {
            break;
        }
        let e2 = error * 2;
        if e2 >= delta_y {
            error += delta_y;
            x += sign_x;
        }
        if e2 <= delta_x {
            error += delta_x;
            y += sign_y;
        }
    }
}

fn draw_column_first_person(distance: f32, frame: &mut [u8], i: usize, side: usize) {
    let wall_height = ((SCREEN_HEIGHT as f32 / distance) as usize).min(SCREEN_HEIGHT);

    let draw_start = (SCREEN_HEIGHT - wall_height) / 2;
    let draw_end = SCREEN_HEIGHT - draw_start;

    for j in 0..SCREEN_HEIGHT {
        let color = if j >= draw_start && j < draw_end && distance < Z_FAR {
            COLOR_FADEAWAY / (distance + COLOR_FADEAWAY) * if side == 0 {
                EW_WALL_COLOR
            } else {
                NS_WALL_COLOR
            }
        } else {
            let num = SCREEN_HEIGHT - 2 * j;
            let floor_scalar = (num * num) as f32 / (SCREEN_HEIGHT * SCREEN_HEIGHT) as f32;
            FLOOR_COLOR * floor_scalar
        };
        draw(frame, i, j, color);
    }
}

fn draw_player(screen_position: Vec2, frame: &mut [u8]) {
    if DEBUG_TOPDOWN {
        for j in 0..SCREEN_HEIGHT {
            for i in 0..SCREEN_WIDTH {
                if screen_position.distance_squared(Vec2::new(i as f32, j as f32)) < RADIUS_SQUARED
                {
                    draw(frame, i, j, TOPDOWN_PLAYER_COLOR);
                }
            }
        }
    }
}

fn draw(frame: &mut [u8], x: usize, y: usize, color: Vec3) {
    let slice = [color.x, color.y, color.z, 1.0];
    for (k, linear) in slice.iter().enumerate() {
        frame[COLOR_DEPTH * (y * SCREEN_WIDTH + x) + k] =
            (((COLOR_MAXVAL as f32 * linear) as u8) as f32 * MAXVAL_MAX as f32 / COLOR_MAXVAL as f32) as u8;
    }
}
