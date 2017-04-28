use risp::eval_risp_script;
use risp::types::{RispError, error};
use risp::core::create_core_environment;
use utils::read_file;

use midi_devices::*;

pub struct Config {
    pub view: bool,
    pub selected_patch: String,
    pub default_in_device: String,
    pub default_out_device: String
}

impl Config {
    fn get_default() -> Self {
        Config {
            view: true ,
            selected_patch: "".to_string(),
            default_in_device: DEFAULT_IN_DEVICE.to_string(),
            default_out_device: DEFAULT_OUT_DEVICE.to_string()
        }
    }
}

pub fn load_config(file_name: &str) -> Result<Config, RispError> {
    let risp_code = read_file(file_name).map_err(|_| error(format!("Can't read file {:?}", file_name)))?;
    let mut env = create_core_environment();
    let evaluated_config = eval_risp_script(&risp_code, &mut env)?;
    let default_config = Config::get_default();

    Ok(
        Config {
            view: evaluated_config.get("view")?.unwrap_or(default_config.view),
            selected_patch: evaluated_config.get("selected_patch")?.unwrap_or(default_config.selected_patch),
            default_in_device: evaluated_config.get("default_in_device")?.unwrap_or(default_config.default_in_device),
            default_out_device: evaluated_config.get("default_out_device")?.unwrap_or(default_config.default_out_device),
        }
    )
}