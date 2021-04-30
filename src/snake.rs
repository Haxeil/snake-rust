use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNALKE_COLOR : Color = [0.20, 0.25, 0.00, 1.0];

pub struct Snake
{
    direction : Direction,
    body : LinkedList<Block>,
    tail : Option<Block>,


}

impl Snake
{

    pub fn new(x : i32, y : i32) -> Self
    {
        let mut body : LinkedList<Block> = LinkedList::new();
        body.push_back(
            Block
            {
                x : x + 2,
                y,
            }
        );
        body.push_back(
            Block
            {
                x : x + 1,
                y,
            }
        );
        body.push_back(
            Block
            {
                x,
                y,
            }
        );
        Snake
        {
            direction : Direction::Right,
            body,
            tail : None,
        }

    }

}

impl Snake
{
    pub fn draw(&self, con : &Context, g : &mut G2d)
    {
        for block in &self.body
        {
            draw_block(SNALKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32)
    {
        let head_block = match self.body.front()
        {
            Some(block) => block,
            None => &Block
            {
                x : -1,
                y : -1,
            },
        };


        (head_block.x, head_block.y)
    }
    pub fn move_forward(&mut self, dir : Option<Direction>)
    {
        match dir 
        {
            Some(d) => self.direction = d,
            None => (),
        }
        let (last_x, last_y) : (i32, i32) = self.head_position();

        let new_block = match self.direction
        {
            Direction::Up => Block
            {
                x : last_x,
                y : last_y - 1,
            },
            Direction::Down => Block
            {
                x : last_x,
                y : last_y + 1,
            },
            Direction::Right => Block
            {
                x : last_x + 1,
                y : last_y,
            },
            Direction::Left => Block
            {
                x : last_x - 1,
                y : last_y,
            },
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();

        self.tail = Some(removed_block);


    }
}

pub enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

impl Direction
{

    pub fn opposite(&self) -> Direction
    {

        match self
        {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }

    }

}

struct Block
{
    x : i32,
    y : i32,
}

