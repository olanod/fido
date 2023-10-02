use wasm_bindgen::JsCast;

pub struct GetElement<T>(T);

impl<T> GetElement<T> {
    pub fn get_element_by_id(tag: &str) -> T
    where
        T: JsCast,
    {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let element = document
            .get_element_by_id(tag)
            .unwrap()
            .dyn_into::<T>()
            .unwrap();

        element
    }

    pub fn query_selector(selectors: &str) -> T
    where
        T: JsCast,
    {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");

        let elements = document
            .query_selector_all(&selectors)
            .unwrap()
            .dyn_into::<T>()
            .unwrap();

        elements
    }
}
