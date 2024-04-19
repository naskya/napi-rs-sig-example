#[napi_derive::napi(object, js_name = "Foo")]
pub struct Thing {
    pub name: String,
}
