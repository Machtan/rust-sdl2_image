extern crate sdl2;
extern crate sdl2_image;

use std::env;
use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        return println!("Usage: cargo run /path/to/image.(png|jpg)");
    }
    let path = Path::new(&args[1]);
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // Initialize the sdl2_image context.
    let image_context = sdl2_image::init().png().jpg().finish().unwrap();
    
    
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
      .position_centered()
      .build()
      .unwrap();

    let mut renderer = window.renderer().software().build().unwrap();
    
    // Load the image as a texture
    let texture = image_context.load_texture(&renderer, &path).unwrap();
    
    // Color the background white, in case the image is transparent
    renderer.set_draw_color(Color::RGBA(255, 255, 255, 255));
    renderer.clear();
    
    // Draw the texture with the image on it
    renderer.copy(&texture, None, None);
    
    // Show the results of the drawing on the screen
    renderer.present();

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }
}
