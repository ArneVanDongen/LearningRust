use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod test;

fn generic_function<Type1:ToString>(arg: Type1) -> Result<String, String> {
    return Ok(arg.to_string());
}

pub fn main() -> Result<(), String> {


    let some = generic_function("test");
    println!("Some result: {}", some?);

    test::other_method();

    let sdl_result = sdl2::init();

    let sdl_context = match sdl_result {
        Ok(result) => result,
        Err(message) => {
            println!("SDL reported an error: '{}'", message);
            return Ok(());
        }
    };

    let video_subsystem = sdl_context.video()?;
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}
