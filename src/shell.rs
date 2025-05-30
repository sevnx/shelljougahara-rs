use crate::fs::FileSystem;

pub struct Shell {
    pub fs: FileSystem,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            fs: FileSystem::default(),
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

impl Shell {
    pub fn execute(&self, command: &str) -> Result<(), String> {
        if command.is_empty() {
            return Err("Tried to execute an empty command".to_string());
        }
        // let tokens = shlex::split(command).ok_or("Failed to split command")?;
        // let command = tokens.first().ok_or("Failed to get command")?;

        Ok(())
    }
}
