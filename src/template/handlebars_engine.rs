use crate::template::engine::Engine;
use handlebars::Handlebars;
use std::collections::BTreeMap;

pub struct HandlebarsEngine {

}

impl Engine for HandlebarsEngine {
    fn render(&mut self, template: &str, context: super::context::Context) -> String {
        context.content;
        context.metadata.author;
        context.metadata.title;
        context.metadata.date;
        context.metadata.description;
        context.metadata.time;

        // take template from render and create a handlebars template out of it
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("template_0", template).unwrap();

        // register handlebars template => parse -> compile -> cache in registry

    }
}
