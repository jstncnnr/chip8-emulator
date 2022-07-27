use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use chip8::emulator::Emulator;
use chip8::rom::ROM;

fn main() {
    let rom = ROM::from_file("./data/test_opcode.ch8").expect("Error opening ROM");
    let mut emulator = Emulator::new(&rom);

    let scale: i32 = 5;
    let sdl_context = sdl2::init().unwrap();
    let video_ctx = sdl_context.video().unwrap();
    let window = video_ctx.window("CHIP-8", (64 * scale) as u32, (32 * scale) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    emulator.start();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        emulator.cycle();

        if emulator.fb_needs_refresh() {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            let fb_data = emulator.get_fb_data();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            for y in 0..32 {
                for x in 0..64 {
                    if fb_data[y * 64 + x] == 1 {
                        canvas.fill_rect(Rect::new((x as i32) * scale, (y as i32) * scale, scale as u32, scale as u32)).unwrap();
                    }
                }
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    emulator.stop();
}
