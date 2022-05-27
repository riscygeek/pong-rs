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
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

pub mod ball;
pub mod flapper;

pub fn transform_rect(canvas: &WindowCanvas, x: f32, y: f32, w: f32, h: f32) -> Rect {
    let (ww, wh) = canvas.window().size();
    let ww = ww as f32;
    let wh = wh as f32;
    Rect::new(
        (x * ww) as i32,
        (y * wh) as i32,
        (w * ww) as u32,
        (h * wh) as u32,
    )
}
