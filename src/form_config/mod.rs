use crate::input_validator::NonEmptyInputValidator;
use tera::Context;

pub trait FormConfig {
    fn context(&mut self) -> &mut Context;
    fn input_validator(&self) -> &NonEmptyInputValidator;
    fn message_body(&self) -> String;
    fn message_printed(&mut self) -> &mut bool;
    fn name(&self) -> String;
    fn set_message_body(&mut self, message_body: String);
    fn set_name(&mut self, name: String);
}

pub struct FormConfigImpl {
    context: Context,
    input_validator: NonEmptyInputValidator,
    message_body: String,
    message_printed: bool,
    name: String,
}

impl FormConfigImpl {
    pub fn new() -> Self {
        let input_validator = NonEmptyInputValidator;
        let mut context = Context::new();
        context.insert("name", "User");
        context.insert("context", "Rust htmx Demo");

        Self {
            input_validator,
            message_printed: false,
            name: String::new(),
            message_body: String::new(),
            context,
        }
    }
}

impl FormConfig for FormConfigImpl {
    fn input_validator(&self) -> &NonEmptyInputValidator {
        &self.input_validator
    }

    fn message_printed(&mut self) -> &mut bool {
        &mut self.message_printed
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn message_body(&self) -> String {
        self.message_body.clone()
    }

    fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_message_body(&mut self, message_body: String) {
        self.message_body = message_body;
    }
}
