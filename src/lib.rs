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
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use std::rc::Rc;
use std::cell::RefCell;

use sdl2::event::Event as SDLEvent;
use sdl2::video::GLContext;

use tuber::window::{Window, WindowEvent};

mod input;
use crate::input::keyboard::SDLKey;

pub struct SDLWindow {
    sdl_window: sdl2::video::Window,
    sdl_event_pump: Rc<RefCell<sdl2::EventPump>>,
    _gl_context: GLContext,
}

impl SDLWindow {
    pub fn new(sdl_video_subsystem: &sdl2::VideoSubsystem, 
               sdl_event_pump: Rc<RefCell<sdl2::EventPump>>) -> SDLWindow {
        let sdl_window = sdl_video_subsystem.window("Untitled Window", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .expect("Window creation failed");

        let _gl_context = sdl_window.gl_create_context()
            .expect("GL context creation failed");

        SDLWindow {
            sdl_window,
            sdl_event_pump,
            _gl_context
        }
    }
}

impl Window for SDLWindow {

    fn display(&mut self) {
        self.sdl_window.gl_swap_window();
    }

    fn poll_event(&mut self) -> Option<WindowEvent> {
        if let Some(sdl_event) = self.sdl_event_pump.borrow_mut().poll_event() {
            return Some(match sdl_event {
                SDLEvent::Quit{..} => 
                    WindowEvent::Close,
                SDLEvent::KeyDown{keycode: Some(k), ..} => 
                    WindowEvent::KeyDown(SDLKey(k).into()),
                SDLEvent::KeyUp{keycode: Some(k), ..} => 
                    WindowEvent::KeyDown(SDLKey(k).into()),
                _ => 
                    WindowEvent::Unknown
            });
        }

        None
    }
}

