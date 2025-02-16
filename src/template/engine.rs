use super::context::Context;

pub trait Engine {
   fn render(&mut self, template: &str, context: Context) -> String;
}
