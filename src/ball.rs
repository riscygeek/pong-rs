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
use crate::flapper::Flapper;
use crate::transform_rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use rand::Rng;

pub struct Ball {
    x: f32,
    y: f32,
    angle: f32,
    speed: f32,
    hits: u32,
}

impl Ball {
    const INITIAL_SPEED: f32 = 0.5;
    const MAX_SPEED: f32 = 7.0;
    pub const WIDTH: f32 = 0.04;
    pub const HEIGHT: f32 = 0.05;
    
    fn rand_angle() -> f32 {
        let min = 0.4f32;
        let mut rng = rand::thread_rng();
        rng.gen_range(min..(std::f32::consts::PI - min)) * (rng.gen_range(1..3) as f32)
    }
    pub fn new() -> Ball {
        Ball {
            x: 0.5,
            y: 0.5,
            angle: Self::rand_angle(),
            speed: Self::INITIAL_SPEED,
            hits: 0,
        }
    }
    
    pub fn reset(&mut self) {
        self.x = 0.5;
        self.y = 0.5;
        self.angle = Self::rand_angle();
        self.speed = Self::INITIAL_SPEED;
        self.hits = 0;
    }
    
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn hits(&self) -> u32 { self.hits }
    
    fn collides_with(&self, f: &Flapper) -> bool {
        self.x >= (f.x() - Self::WIDTH) && self.x <= (f.x() + Flapper::WIDTH) &&
        self.y >= (f.y - Self::HEIGHT) && self.y <= (f.y + f.height)
    }
    pub fn update(&mut self, dt: f32, flappers: &mut [Flapper]) -> Result<(), u8> {
        let mut dx = self.angle.sin();
        let mut dy = self.angle.cos();
        
        if self.x >= (0.98 - Self::WIDTH) {
            return Err(0);
        } else if self.x <= 0.02 {
            return Err(1);
        } else if self.y >= (0.98 - Self::HEIGHT) || self.y <= 0.02 {
            dy = -dy;
            self.angle = dx.atan2(dy);
        }
        
        let mut hit = false;
        for f in flappers.iter() {
            if self.collides_with(f) {
                dx = -dx;
                self.angle = dx.atan2(dy);
                self.hits += 1;
                hit = true;
            }
        }
        if hit {
            for f in flappers.iter_mut() {
                f.height = (f.height * 0.985).max(0.002);
            }
            self.speed = (self.speed * 1.01).min(Self::MAX_SPEED);
        }
        
        self.x = (self.x + dx * self.speed * dt).clamp(0.0, 1.0 - Self::WIDTH);
        self.y = (self.y - dy * self.speed * dt).clamp(0.0, 1.0 - Self::HEIGHT);
        Ok(())
    }
    
    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let rect = transform_rect(canvas, self.x, self.y, Self::WIDTH, Self::HEIGHT);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(rect)
    }
}

