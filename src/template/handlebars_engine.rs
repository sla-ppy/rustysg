use crate::template::engine::Engine;
use handlebars::{no_escape, Handlebars};

pub struct HandlebarsEngine {}

impl HandlebarsEngine {
    pub fn new() -> Self {
        HandlebarsEngine{}
    }
}

impl Engine for HandlebarsEngine {
    fn render(&mut self, template: &str, context: super::context::Context) -> String {
        // create handlebars template out of base template
        let mut handlebars = Handlebars::new();
        // register handlebar template => parses -> compiles -> caches in registry
        handlebars
            .register_template_string("handlebars_template", template)
            .unwrap();

        // don't escape html like `<` and `>` since we want to render html here
        handlebars.register_escape_fn(no_escape);
        // gives compiler RenderError, when accessing unexistant fields
        handlebars.set_strict_mode(true);
        // render into string
        handlebars.render("handlebars_template", &context).unwrap()
    }
}
