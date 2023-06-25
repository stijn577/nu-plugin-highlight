use nu_plugin::{serve_plugin, MsgPackSerializer};
use plugin::HighlightPlugin;

mod highlight;
mod plugin;

fn main() {
    serve_plugin(&mut HighlightPlugin::new(), MsgPackSerializer);
}
