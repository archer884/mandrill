#[derive(Deserialize)]
pub struct InfoResponse {
    pub code: String,
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct RenderResponse { pub html: String }
