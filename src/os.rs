
#[derive(Debug)]
pub enum OperatingSystem {
    MacOs,
    Linux,
}

impl OperatingSystem {
    pub fn detect() -> Self {
        match std::env::consts::OS {
            "linux" => OperatingSystem::Linux,
            "macos" => OperatingSystem::MacOs,
            _ => todo!("This OS is not supported"),
        }
    }
}
