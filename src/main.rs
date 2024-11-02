use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoop};

use app::App;

mod app;

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}
