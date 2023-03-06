mod renderer;

use std::time::Duration;

use renderer::{RenderState, SCREEN_WIDTH};
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u32 = 10;
const WINDOW_WIDTH: u32 = SCALE * renderer::SCREEN_WIDTH as u32;
const WINDOW_HEIGHT: u32 = SCALE * renderer::SCREEN_HEIGHT as u32;

fn main() -> Result<(), String> {
    let sdl2_context = sdl2::init()?;
    let video_subsystem = sdl2_context.video()?;

    let window = video_subsystem
        .window("learning sdl2", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl2_context.event_pump()?;

    let mut renderer = renderer::RenderState::new();

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'gameloop,
                Event::KeyDown { .. } => {renderer.draw_to_center(Color::RGB(0, 255, 0))},
                Event::KeyUp { .. } => {renderer.draw_to_center(Color::RGB(0, 0, 0))},
                _ => {}
            }
        }
        canvas.clear();
        canvas.present();
        draw_screen(&renderer, &mut canvas)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

fn draw_screen(renderer: &RenderState, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = renderer.get_display();
    for (i, pixel) in screen_buffer.iter().enumerate() {
        canvas.set_draw_color(*pixel);
        let x = (i % SCREEN_WIDTH) as u32;
        let y = (i / SCREEN_WIDTH) as u32;

        let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
        canvas.fill_rect(rect)?;
    }
    canvas.present();
    Ok(())
}
