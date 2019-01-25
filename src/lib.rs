/*
* MIT License
*
* Copyright (c) 2018 Clément SIBILLE
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

extern crate tuber;
extern crate sdl2;

use tuber::window::{ Window, WindowEvent };

pub struct SDLWindow {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
}

impl SDLWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Result<SDLWindow, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window(title, width, height).build();
        let window = match window {
            Ok(window) => window,
            Err(error) => {
                return Err(error.to_string())
            }
        };

        Ok(SDLWindow {
            sdl_context,
            window
        })
    }
}

impl Window for SDLWindow {
    fn show(&mut self) {
        self.window.show();
    }
    fn hide(&mut self) {
        self.window.hide();
    }

    fn poll_event(&mut self) -> WindowEvent {
        use sdl2::event::Event;

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            return match event {
                Event::Quit {..} => WindowEvent::Close,
                _ => WindowEvent::None
            }
        }

        WindowEvent::None
    }
}