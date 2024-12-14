use rand::{self, random};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Snake")
        .msaa_4x()
        .build();

    rl.set_target_fps(60);

    let mut world = World {
        width: 32,
        height: 24,
        scale: 20,
        tick_delay: 5,
        food: Position { x: 5, y: 16 },
    };
    let mut snake = Snake::new(Position { x: 5, y: 5 }, 5, Direction::Right);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        snake.update(&mut d, &mut world);
        snake.draw(&mut d, &world);
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

struct World {
    width: u32,
    height: u32,
    scale: u32,
    tick_delay: u32,
    food: Position,
}

struct Snake {
    body: Vec<Position>,
    alive: bool,
    direction: Direction,
    ticker: u32,
}

impl Snake {
    fn new(pos: Position, length: u32, dir: Direction) -> Self {
        Snake {
            body: (0..length)
                .map(|n| match dir {
                    Direction::Up => Position {
                        x: pos.x,
                        y: pos.y + n as i32,
                    },
                    Direction::Right => Position {
                        x: pos.x - n as i32,
                        y: pos.y,
                    },
                    Direction::Down => Position {
                        x: pos.x,
                        y: pos.y - n as i32,
                    },
                    Direction::Left => Position {
                        x: pos.x + n as i32,
                        y: pos.y,
                    },
                })
                .collect(),
            alive: true,
            direction: dir,
            ticker: 0,
        }
    }

    fn update(self: &mut Self, handle: &mut RaylibDrawHandle, world: &mut World) {
        self.body.clone().iter().enumerate().for_each(|(i, val)| {
            if i != 0 {
                if val.clone() == self.body[0] {
                    self.alive = false;
                }
            } else {
                if val.clone() == world.food {
                    self.body.push(self.body[self.body.len() - 1]);
                    world.food = Position {
                        x: (random::<i32>() % world.width as i32).abs(),
                        y: (random::<i32>() % world.height as i32).abs(),
                    };

                    while self.body.contains(&world.food) {
                        world.food = Position {
                            x: (random::<i32>() % world.width as i32).abs(),
                            y: (random::<i32>() % world.height as i32).abs(),
                        };
                    };
                }
            }
        });
        let mut dir_queue: Direction = self.direction;

        if handle.is_key_down(KeyboardKey::KEY_UP) && self.direction != Direction::Down {
            dir_queue = Direction::Up;
        }
        if handle.is_key_down(KeyboardKey::KEY_DOWN) && self.direction != Direction::Up {
            dir_queue = Direction::Down;
        }
        if handle.is_key_down(KeyboardKey::KEY_LEFT) && self.direction != Direction::Right {
            dir_queue = Direction::Left;
        }
        if handle.is_key_down(KeyboardKey::KEY_RIGHT) && self.direction != Direction::Left {
            dir_queue = Direction::Right;
        }

        self.direction = dir_queue;
        self.ticker += 1;

        if self.ticker % world.tick_delay == 0 && self.alive {
            let new_body: Vec<Position> = {
                self.body.iter().enumerate().map(|(i, val)| {
                    if i == 0 {
                        match self.direction {
                            Direction::Up => Position {
                                x: val.x,
                                y: emod(val.y - 1, world.height as i32),
                            },
                            Direction::Right => Position {
                                x: emod(val.x + 1, world.width as i32),
                                y: val.y,
                            },
                            Direction::Down => Position {
                                x: val.x,
                                y: emod(val.y + 1, world.height as i32),
                            },
                            Direction::Left => Position {
                                x: emod(val.x - 1, world.width as i32),
                                y: val.y,
                            },
                        }
                    } else {
                        self.body[i - 1]
                    }
                })
            }
            .collect();
            self.body = new_body;
        }
    }

    fn draw(&self, handle: &mut RaylibDrawHandle, world: &World) {
        self.body.iter().for_each(|pos| {
            handle.draw_rectangle(
                pos.x * world.scale as i32,
                pos.y * world.scale as i32,
                world.scale as i32,
                world.scale as i32,
                if self.alive { Color::WHITE } else { Color::RED },
            );
        });

        handle.draw_rectangle(
            world.food.x * world.scale as i32,
            world.food.y * world.scale as i32,
            world.scale as i32,
            world.scale as i32,
            Color::ORANGE,
        );
    }
}

fn emod(l: i32, r: i32) -> i32 {
    ((l % r) + r) % r
}
