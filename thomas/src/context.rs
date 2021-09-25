use super::audio::Audio;
use super::debugger::Debugger;
use super::graphics::backend::State;
use super::keyboard::Keyboard;
use super::resource::ResourceManager;

pub struct Context {
    pub graphics: State,
    pub keyboard: Keyboard,
    pub audio: Audio,
    pub window: winit::window::Window,
    pub resource_mgr: ResourceManager,
    pub config: crate::Config,
    pub debugger: Debugger,
}
