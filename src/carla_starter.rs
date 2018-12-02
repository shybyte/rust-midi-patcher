use std::process::Command;
use libc::{kill, SIGTERM};
use std::process::Child;
use std::path::PathBuf;

pub struct CarlaStarter {
    carla_path: PathBuf,
    child_process: Option<Child>,
    patch_name: String,
}

impl CarlaStarter {
    pub fn new<S: Into<PathBuf>>(carla_path: S) -> Self {
        CarlaStarter {
            carla_path: carla_path.into(),
            child_process: None,
            patch_name: "".to_string()
        }
    }

    pub fn on_patch_change(&mut self, patch_name: &str)  {
        if self.patch_name == patch_name {
            return
        }

        let path = self.carla_path.join(patch_name.to_string() + ".carxp");

        if !path.exists() {
            return;
        }

        if let Some(ref child_process) = self.child_process {
            let child_id = child_process.id();
            unsafe {
                kill(child_id as i32, SIGTERM);
            }
        }

        let child_result = Command::new("carla").arg(path).spawn();

        match child_result {
            Ok(child) => self.child_process = Some(child),
            Err(e) => eprintln!("Error starting carla = {:?}", e)
        }
    }
}

