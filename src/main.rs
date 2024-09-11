use core::engine::platform::window::{self, Window};

pub mod core;

fn main() {
    let platform = window::Window::new(Window {
        size: todo!(),
    });
}
