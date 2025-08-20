use std::collections::HashMap;
use validator::Validate;

pub fn validate_form<T: Validate>(form: &T) -> Result<(), HashMap<String, String>> {
    if let Err(validation_errors) = form.validate() {
        let mut errors = HashMap::new();

        for (field, field_errors) in validation_errors.field_errors() {
            let rule_errors = field_errors
                .iter()
                .map(|error| {
                    error
                        .message
                        .as_deref()
                        .unwrap_or("Invalid input")
                        .to_string()
                })
                .collect::<Vec<String>>()
                .join(", ");

            errors.insert(field.to_string(), rule_errors);
        }

        return Err(errors);
    }

    Ok(())
}
