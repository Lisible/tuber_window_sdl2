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
extern crate gl;

use sdl2::event::WindowEvent;
use sdl2::mouse::MouseButton as SDL2MouseButton;
use sdl2::keyboard::Keycode as SDL2Key;
use tuber::window::Window;
use tuber::input::{Input, keyboard, mouse};

pub struct SDLWindow {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext
}

impl SDLWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Result<SDLWindow, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window(title, width, height).opengl().build();
        let window = match window {
            Ok(window) => window,
            Err(error) => {
                return Err(error.to_string())
            }
        };

        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        // Creating OpenGL context
        let _gl_context = window.gl_create_context()?;

        Ok(SDLWindow {
            sdl_context,
            window,
            _gl_context
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

    fn poll_input(&mut self) -> Option<Input> {
        use sdl2::event::Event;

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            return match event {
                Event::Quit {..} => Some(Input::Close),
                Event::Window {win_event: WindowEvent::Resized(width, height), ..} =>
                    Some(Input::Resize(width as u32, height as u32)),
                Event::KeyDown { keycode: Some(key), .. } =>
                    Some(Input::KeyDown(sdl_key_to_tuber_key(key))),
                Event::KeyUp { keycode: Some(key), .. } =>
                    Some(Input::KeyUp(sdl_key_to_tuber_key(key))),
                Event::MouseButtonDown { mouse_btn: button, .. } =>
                    Some(Input::MouseDown(sdl_mouse_button_to_tuber_mouse_button(button))),
                Event::MouseButtonUp { mouse_btn: button, .. } =>
                    Some(Input::MouseUp(sdl_mouse_button_to_tuber_mouse_button(button))),

                _ => None
            }
        }

        None
    }

    fn set_current_graphics_context(&self) {
        if let Err(x) = self.window.gl_set_context_to_current() {
            eprintln!("{}", x); 
        }
    }

    fn display(&mut self) {
        self.window.gl_swap_window();
    }
}

fn sdl_key_to_tuber_key(key: SDL2Key) -> keyboard::Key {
    match key {
        SDL2Key::A => keyboard::Key::A,
        SDL2Key::B => keyboard::Key::B,
        SDL2Key::C => keyboard::Key::C,
        SDL2Key::D => keyboard::Key::D,
        SDL2Key::E => keyboard::Key::E,
        SDL2Key::F => keyboard::Key::F,
        SDL2Key::G => keyboard::Key::G,
        SDL2Key::H => keyboard::Key::H,
        SDL2Key::I => keyboard::Key::I,
        SDL2Key::J => keyboard::Key::J,
        SDL2Key::K => keyboard::Key::K,
        SDL2Key::L => keyboard::Key::L,
        SDL2Key::M => keyboard::Key::M,
        SDL2Key::N => keyboard::Key::N,
        SDL2Key::O => keyboard::Key::O,
        SDL2Key::P => keyboard::Key::P,
        SDL2Key::Q => keyboard::Key::Q,
        SDL2Key::R => keyboard::Key::R,
        SDL2Key::S => keyboard::Key::S,
        SDL2Key::T => keyboard::Key::T,
        SDL2Key::U => keyboard::Key::U,
        SDL2Key::V => keyboard::Key::V,
        SDL2Key::W => keyboard::Key::W,
        SDL2Key::X => keyboard::Key::X,
        SDL2Key::Y => keyboard::Key::Y,
        SDL2Key::Z => keyboard::Key::Z,
        SDL2Key::Return => keyboard::Key::Return,
        SDL2Key::KpEnter => keyboard::Key::Enter,
        SDL2Key::LShift => keyboard::Key::LShift,
        SDL2Key::RShift => keyboard::Key::RShift,
        SDL2Key::LCtrl => keyboard::Key::LControl,
        SDL2Key::RCtrl => keyboard::Key::RControl,
        SDL2Key::Escape => keyboard::Key::Escape,
        _ => keyboard::Key::Unknown,
    }
}

fn sdl_mouse_button_to_tuber_mouse_button(button: SDL2MouseButton) -> mouse::Button {
    match button {
        SDL2MouseButton::Left => mouse::Button::Left,
        SDL2MouseButton::Right => mouse::Button::Right,
        SDL2MouseButton::Middle => mouse::Button::Middle,
        _ => mouse::Button::Unknown,
    }
}
