use poem_openapi::Object;

#[derive(Object)]
pub struct ValidateInput {
    pub run: String
}