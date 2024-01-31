use std::collections::HashMap;

pub fn prepare_form_values(raw_values: HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut prepared_values: HashMap<String, String> = HashMap::new();

    raw_values.into_iter().for_each(|(key, value)| {
        prepared_values.insert(key, value.join(""));
    });

    prepared_values
}