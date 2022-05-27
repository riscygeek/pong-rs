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
use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;

use pong::ball::Ball;
use pong::flapper::Flapper;
use pong::transform_rect;

fn font_path() -> &'static str {
    let path = ".bit5x3.ttf";
    let font = include_bytes!("bit5x3.ttf");
    let mut file = File::create(path).unwrap();
    file.write_all(font).unwrap();
 
    path
}

fn main() -> Result<(), String> {
    let sdl2 = sdl2::init().unwrap();
    let ttf = sdl2::ttf::init().unwrap();
    let video = sdl2.video().unwrap();
    
    let font_path = font_path();
    let font = ttf.load_font(font_path, 128)?;
    
    let window = video.window("Pong", 800, 600)
        .resizable()
        .build()
        .unwrap();
        
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();
    
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    
    let mut ball = Ball::new();
    let mut flappers = [Flapper::new(0), Flapper::new(1)];
    let mut events = sdl2.event_pump().unwrap();
    let mut last_time = Instant::now() - Duration::from_nanos(1);
    let mut updown = [false; 4];
    let players = 0;
    'running: loop {
        let now = Instant::now();
        let dt = ((now - last_time).as_micros() as f32) / 1.0e6;
        
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    updown[0] = true;
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    updown[0] = false;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    updown[1] = true;
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    updown[1] = false;
                },
                
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    updown[2] = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    updown[2] = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    updown[3] = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    updown[3] = false;
                },
                
                Event::Window { win_event: WindowEvent::Resized(w, _), .. } => {
                    let w = w as u32;
                    canvas.window_mut().set_size(w, w * 6 / 8).unwrap();
                },
                _ => {},
            }
        }
        
        if players >= 1 {
            if updown[0] {
                flappers[0].y -= dt;
            } 
            if updown[1] {
                flappers[0].y += dt;
            }
        } else {
            flappers[0].update(&ball);
        }
        
        if players == 2 {
            if updown[2] {
                flappers[1].y -= dt;
            } 
            if updown[3] {
                flappers[1].y += dt;
            }
        } else {
            flappers[1].update(&ball);
        }
        if let Err(side) = ball.update(dt, &mut flappers) {
            flappers[side as usize].score += 1;
            ball.reset();
            for f in &mut flappers {
                f.reset();
            }
        }
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Draw the border.
        canvas.set_draw_color(Color::RGB(32, 32, 32));
        let rects = [
            transform_rect(&canvas, 0.00, 0.00, 1.00, 0.02),
            transform_rect(&canvas, 0.00, 0.98, 1.00, 0.02),
            transform_rect(&canvas, 0.00, 0.00, 0.02, 1.00),
            transform_rect(&canvas, 0.98, 0.00, 0.02, 1.00),
            transform_rect(&canvas, 0.495, 0.00, 0.01, 1.00),
        ];
        canvas.fill_rects(&rects)?;
        
        // Draw the score.
        let surface = font.render(&format!("{}:{}", flappers[0].score, flappers[1].score)).solid(Color::RGB(128, 128, 128)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let target = transform_rect(&canvas, 0.41, 0.03, 0.2, 0.125);
        canvas.copy(&texture, None, Some(target))?;
        
        // Draw the ball and the flappers.
        ball.draw(&mut canvas)?;
        for f in &flappers {
            f.draw(&mut canvas)?;
        }
        
        canvas.present();
        last_time = now;
    }
    
    std::fs::remove_file(font_path).unwrap();
    
    Ok(())
}
