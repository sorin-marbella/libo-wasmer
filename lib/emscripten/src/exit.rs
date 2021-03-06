use crate::EmEnv;

// __exit
pub fn exit(_ctx: &mut EmEnv, value: i32) {
    debug!("emscripten::exit {}", value);
    ::std::process::exit(value);
}
