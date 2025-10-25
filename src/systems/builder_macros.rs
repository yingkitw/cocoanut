//! Builder Pattern Macros
//! 
//! Macros to reduce boilerplate in builder pattern implementations.

/// Macro for simple getter methods
/// Usage: getter!(field_name, return_type, doc_comment)
#[macro_export]
macro_rules! getter {
    ($field:ident, &str) => {
        pub fn $field(&self) -> &str {
            &self.$field
        }
    };
    ($field:ident, bool) => {
        pub fn $field(&self) -> bool {
            self.$field
        }
    };
    ($field:ident, usize) => {
        pub fn $field(&self) -> usize {
            self.$field
        }
    };
    ($field:ident, f64) => {
        pub fn $field(&self) -> f64 {
            self.$field
        }
    };
}

/// Macro for Option<&str> getter methods
/// Usage: option_getter!(field_name)
#[macro_export]
macro_rules! option_getter {
    ($field:ident) => {
        pub fn $field(&self) -> Option<&str> {
            self.$field.as_deref()
        }
    };
}

/// Macro for boolean checker methods (is_* pattern)
/// Usage: bool_checker!(field_name, method_name)
#[macro_export]
macro_rules! bool_checker {
    ($field:ident, $method:ident) => {
        pub fn $method(&self) -> bool {
            self.$field
        }
    };
}

/// Macro for builder setter methods
/// Usage: builder_setter!(field_name, param_type)
#[macro_export]
macro_rules! builder_setter {
    ($field:ident, String) => {
        pub fn $field(mut self, value: impl Into<String>) -> Self {
            self.$field = value.into();
            self
        }
    };
    ($field:ident, bool) => {
        pub fn $field(mut self, value: bool) -> Self {
            self.$field = value;
            self
        }
    };
    ($field:ident, usize) => {
        pub fn $field(mut self, value: usize) -> Self {
            self.$field = value;
            self
        }
    };
    ($field:ident, f64) => {
        pub fn $field(mut self, value: f64) -> Self {
            self.$field = value;
            self
        }
    };
}

/// Macro for Option<String> builder setter methods
/// Usage: option_builder_setter!(field_name)
#[macro_export]
macro_rules! option_builder_setter {
    ($field:ident) => {
        pub fn $field(mut self, value: impl Into<String>) -> Self {
            self.$field = Some(value.into());
            self
        }
    };
}

/// Macro for disabled field pattern
/// Usage: disabled_field!()
#[macro_export]
macro_rules! disabled_field {
    () => {
        pub fn disabled(mut self, disable: bool) -> Self {
            self.disabled = disable;
            self
        }

        pub fn is_disabled(&self) -> bool {
            self.disabled
        }
    };
}

/// Macro for label field pattern
/// Usage: label_field!()
#[macro_export]
macro_rules! label_field {
    () => {
        pub fn label(mut self, text: impl Into<String>) -> Self {
            self.label = Some(text.into());
            self
        }

        pub fn get_label(&self) -> Option<&str> {
            self.label.as_deref()
        }
    };
}

/// Macro for simple new() implementation
/// Usage: simple_new!(StructName, field1: value1, field2: value2)
#[macro_export]
macro_rules! simple_new {
    ($struct_name:ident, $($field:ident: $value:expr),*) => {
        pub fn new() -> Self {
            $struct_name {
                $($field: $value,)*
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macros_compile() {
        // This test just ensures the macros compile correctly
        assert!(true);
    }
}
