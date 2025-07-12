use sdl2::Sdl;

pub struct SDL {
    // context: Sdl,
    pub window: sdl2::video::Window,
    pub event_pump: sdl2::EventPump,
}

impl SDL {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let context: Sdl = sdl2::init().unwrap();
        let vid = context.video().unwrap();
        let win: sdl2::video::Window = vid.window(&name, width, height).build().unwrap();
        let eventpump: sdl2::EventPump = context.event_pump().unwrap();

        SDL {
            // context: context,
            window: win,
            event_pump: eventpump,
        }
    }
}
