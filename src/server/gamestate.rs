#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Initializing,
    MapGeneration,
    Running,
}
