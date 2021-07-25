use std::collections::LinkedList; // useful to take the head and tail
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        // to not allow the snake to go to opposite direction!
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>, // defines the entire body of the snake, connected with all blocks - that's why we use linked list, because we can add and delete as we want
    tail: Option<Block>, // when we eat the apple
}

impl Snake {
    pub fn new(x:i32, y:i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        // we add 3 blocks in total: at x, x+1 and x+2
        body.push_back(Block {
            x: x + 2,
            y,
        });

        body.push_back(Block {
            x: x + 2,
            y,
        });

        body.push_back(Block {
            x: x + 2,
            y,
        });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,

        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            // this is the reason why we pass an Option
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y) : (i32, i32) = self.head_position();
        
        // when we move in a direction, we want to create a new block and delete the end of the element
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y -1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x -1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        /* 
        when we return an element from an immutable reference, 
        essentially we are cloning it; so we need to implement the clone trait.
        */

        self.direction 
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    // this method is called when we change the direction
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;

        match dir {
            // if dir is Some with a d field, make it equal to moving_dir
            Some(d) => moving_dir = d,
            None => {}
        }
        
        // move the head in any position
        match moving_dir {
            Direction::Up => (head_x, head_y -1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }

    }

    pub fn restore_tail(&mut self) {
        // the tail is not rendered until we eat an apple; after eating it, we create the block
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() -1 {
                break;
            }
        }

        return false;
    }
}