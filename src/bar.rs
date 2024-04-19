#[napi_derive::napi(object, js_name = "Bar")]
pub struct Thing {
    pub id: u32,
}
