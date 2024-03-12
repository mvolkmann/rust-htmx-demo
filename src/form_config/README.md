# Form Configuration Module

This module provides a global form configuration and handler. It allows you to set and retrieve form data such as name, and message body. It also provides methods for validating non-empty input.

## Usage

To use this module, you need to create an instance of `FormConfigImpl`:

```rust
use crate::form_config::{FormConfig, FormConfigImpl};
// ..
let mut form_config = FormConfigImpl::new();
```

You can retrieve the form data using the corresponding getter methods:

```rust
let name = form_config.name();
let message_body = form_config.message_body();
```

The module also provides an `input_validator` method that returns a reference to a NonEmptyInputValidator instance.

## Adding Custom Variables

To add custom variables to the form configuration, you can modify the `FormConfigImpl` struct and add new fields. You can then implement the corresponding setter and getter methods in the `FormConfig` trait and provide an implementation in the `FormConfigImpl` struct.

For example, to add a phone number field, you can do the following:

```rust
pub trait FormConfig {
    // ...
    fn set_phone(&mut self, phone: String);
    fn phone(&self) -> String;
}

pub struct FormConfigImpl {
    // ...
    phone: String,
}

impl FormConfig for FormConfigImpl {
    // ...
    fn set_phone(&mut self, phone: String) {
        self.phone = phone;
    }

    fn phone(&self) -> String {
        self.phone.clone()
    }
}
```

### Note that adding config variables like SMS-service requires adding them as env. variables in .env file and you should change the `const` of default config to your own:

```rust
const DEFAULT_CONFIG: &str =
r#"
SERVER_IP=your_ip
#some of your data
```

## Context

The `FormConfigImpl` struct also contains a `context` field that holds a `Context` instance from the `tera` crate. You can access and modify this context using the `context` method:

```rust
let mut context = form_config.context();
context.insert("key", "value");
```

> This allows you to add custom data to the context that can be used when rendering templates.

## Validation

The module provides a non-empty `input validator`. You can access the validator using the `input_validator` method:

```rust
let input_validator = form_config.input_validator();
```

You can also then use these validators to validate form data:

```rust
let is_name_valid = input_validator.is_valid(&form_config.name());
```
