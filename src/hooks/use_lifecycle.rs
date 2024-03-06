use dioxus::prelude::*;

pub fn use_lifecycle<C: FnOnce(), D: FnOnce() + 'static>(
    cx: &ScopeState,
    create: C,
    destroy: D,
) -> &LifeCycle<D> {
    cx.use_hook(|| {
        create();
        LifeCycle {
            ondestroy: Some(destroy),
        }
    })
}

pub struct LifeCycle<D: FnOnce()> {
    ondestroy: Option<D>,
}

impl<D: FnOnce()> Drop for LifeCycle<D> {
    fn drop(&mut self) {
        let f = self.ondestroy.take().unwrap();
        f();
    }
}
