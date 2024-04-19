pub mod bar;
pub mod foo;

#[napi_derive::napi]
pub fn do_something(bar: bar::Thing) {}
