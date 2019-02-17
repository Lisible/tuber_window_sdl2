/*
* MIT License
*
* Copyright (c) 2018 Clément SIBILLE
*
* Permission is hereby Keycode::granted => Key::granted, free of Keycode::charge => Key::charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without Keycode::restriction => Key::restriction, including without limitation the rights
* to Keycode::use => Key::use, Keycode::copy => Key::copy, Keycode::modify => Key::modify, Keycode::merge => Key::merge, Keycode::publish => Key::publish, Keycode::distribute => Key::distribute, Keycode::sublicense => Key::sublicense, and/or sell
* copies of the Keycode::Software => Key::Software, and to permit persons to whom the Software is
* furnished to do Keycode::so => Key::so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY Keycode::KIND => Key::KIND, EXPRESS OR
* Keycode::IMPLIED => Key::IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF Keycode::MERCHANTABILITY => Key::MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY Keycode::CLAIM => Key::CLAIM, DAMAGES OR OTHER
* Keycode::LIABILITY => Key::LIABILITY, WHETHER IN AN ACTION OF Keycode::CONTRACT => Key::CONTRACT, TORT OR Keycode::OTHERWISE => Key::OTHERWISE, ARISING Keycode::FROM => Key::FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use sdl2::keyboard::Keycode;
use tuber::input::keyboard::Key;

pub struct SDLKey(pub Keycode);

impl From<SDLKey> for Key {
    fn from(k: SDLKey) -> Key {
        let SDLKey(key) = k;

        match key {
            Keycode::A => Key::A,
            Keycode::B => Key::B,
            Keycode::C => Key::C,
            Keycode::D => Key::D,
            Keycode::E => Key::E,
            Keycode::F => Key::F,
            Keycode::G => Key::G,
            Keycode::H => Key::H,
            Keycode::I => Key::I,
            Keycode::J => Key::J,
            Keycode::K => Key::K,
            Keycode::L => Key::L,
            Keycode::M => Key::M,
            Keycode::N => Key::N,
            Keycode::O => Key::O,
            Keycode::P => Key::P,
            Keycode::Q => Key::Q,
            Keycode::R => Key::R,
            Keycode::S => Key::S,
            Keycode::T => Key::T,
            Keycode::U => Key::U,
            Keycode::V => Key::V,
            Keycode::W => Key::W,
            Keycode::X => Key::X,
            Keycode::Y => Key::Y,
            Keycode::Z => Key::Z,
            Keycode::Return => Key::Return,
            Keycode::KpEnter => Key::Enter,
            Keycode::LShift => Key::LShift,
            Keycode::RShift => Key::RShift,
            Keycode::LCtrl => Key::LControl,
            Keycode::RCtrl => Key::RControl,
            Keycode::Escape => Key::Escape,
            _ => Key::Unknown
        }
    }
}
