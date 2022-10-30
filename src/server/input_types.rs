use poem_openapi::Object;

#[derive(Object)]
pub struct ValidationInput {
    pub run: String
}