use web_sys::{window, UrlSearchParams};

pub fn get_param(param: &str) -> Option<String> {
    window()?
        .location()
        .search()
        .ok()
        .map(|search| {
            let params = UrlSearchParams::new_with_str(&search).ok()?;
            params.get(param)
        })
        .flatten()
}
