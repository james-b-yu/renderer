mod app;
mod state;
use app::App;
use renderer::run;
use winit::event_loop::EventLoop;

fn main() {
    println!("Hello, world!");
    run();
}
