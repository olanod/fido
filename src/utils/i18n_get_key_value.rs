use std::collections::HashMap;

pub fn i18n_get_key_value(i18n_map: &HashMap<&str, String>, key: &str) -> String {
    i18n_map.get_key_value(key).unwrap().1.clone()
}
