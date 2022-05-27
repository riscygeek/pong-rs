/*  
  Copyright (C) 2022 Benjamin Stürz

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use crate::transform_rect;
use crate::ball::Ball;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub struct Flapper {
    pub y: f32,
    pub height: f32,
    pub score: u32,
    side: u8,
}

impl Flapper {
    pub const WIDTH: f32 = 0.025;
    pub fn new(side: u8) -> Flapper {
        Flapper {
            y: 0.25,
            height: 0.5,
            score: 0,
            side,
        }
    }
    pub fn reset(&mut self) {
        self.y = 0.25;
        self.height = 0.5;
    }
    
    pub fn x(&self) -> f32 {
        if self.side == 0 {
            Self::WIDTH / 2.0
        } else {
            1.0 - Self::WIDTH * 1.5
        }
    }
    pub fn update(&mut self, ball: &Ball) {
        self.y = ball.y() - self.height * 0.5 + Ball::HEIGHT * 0.5;
    }
    
    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let rect = transform_rect(canvas, self.x(), self.y, Self::WIDTH, self.height);
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.fill_rect(rect)
    }
}
