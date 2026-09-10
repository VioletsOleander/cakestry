use super::state::Mode;

pub enum Action {
    LaunchRequest,
    LaunchCommand,
    SwitchMode(Mode),
    None,
}
