extern crate sdl2 ;

use std::time::{ Duration, Instant } ;
use rand::prelude::* ;
use sdl2::pixels::Color ;
use sdl2::event::Event ;
use sdl2::keyboard::Keycode ;
use sdl2::rect::Rect ;

struct Color_t {
    r:u8,
    g:u8,
    b:u8,
}

impl Default for Color_t {

    fn default() -> Self {
        Color_t {
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

impl Color_t {  

    fn new( x:u8, y:u8, z:u8 ) -> Self {
        Color_t {
            r: x,
            g: y,
            b: z,
        }
    }

    fn randomize( &mut self, rnd_thread: &mut ThreadRng ) {
        self.r = rnd_thread.gen_range( 0..255u8 ) ;
        self.g = rnd_thread.gen_range( 0..255u8 ) ;
        self.b = rnd_thread.gen_range( 0..255u8 ) ;
    }
}

fn main() -> Result< (), String > {    

    let sdl_context = sdl2::init()? ;
    let video_subsystem = sdl_context.video()? ;

    let window = video_subsystem.window( "Window Caption", 1280, 720 )
        .position_centered()
        .build()
        .unwrap() ;        

    let mut canvas = window.into_canvas().build().unwrap() ;
    
    canvas.clear() ;
    canvas.present() ;    

    let mut rnd_thread = rand::thread_rng() ;    
    let mut rect_color = Color_t::default() ;    

    let mut event_pump = sdl_context.event_pump()? ;        
    
    let mut elapsed_time = Instant::now() ;

    'running: loop {        

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }        

        if elapsed_time.elapsed().as_secs() == 2 {
            
            rect_color.randomize( &mut rnd_thread ) ;
            elapsed_time = Instant::now() ;
        }

        canvas.set_draw_color( Color::RGB( rect_color.r, rect_color.g, rect_color.b ) ) ;
        canvas.fill_rect( Rect::new( 0, 0, 50, 50 ) )? ;

        canvas.present() ;
        ::std::thread::sleep( Duration::new( 0, 1_000_000_000u32 / 60 )) ;
    }    

    Ok(())
}