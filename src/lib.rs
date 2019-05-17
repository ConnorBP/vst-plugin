#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;

use vst::plugin::{Info, Plugin, PluginParameters};

//#[derive(Default)]
struct BasicPlugin {
    threshold: f32,
}

impl Default for BasicPlugin {
    fn default() -> BasicPlugin {
        BasicPlugin { threshold: 1.0 }
    }
}

impl Plugin for BasicPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1337, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }
}

impl PluginParameters for BasicPlugin {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold,
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        //        match index {
        //            // We don't want to divide by zero, so we'll clamp the value
        //            0 => self.threshold = value.max(0.01),
        //            _ => (),
        //        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.threshold * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            _ => "".to_string(),
        }
    }
}

plugin_main!(BasicPlugin); // Important!

#[cfg(test)]
mod tests {

    use super::*;
    use vst::plugin::HostCallback;

    #[test]
    fn it_works() {
        assert_eq!(
            BasicPlugin::new(HostCallback::default()).get_info().name,
            "Basic Plugin".to_string()
        );
    }
}
