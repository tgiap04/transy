mod client;

#[derive(Debug)]
pub enum TranslationError {
    Network(reqwest::Error),
    Parse,
    EmptyResponse,
}

impl TranslationError {
    pub fn to_vietnamese(&self) -> &'static str {
        match self {
            Self::Network(_) => "Không có kết nối mạng",
            Self::Parse | Self::EmptyResponse => "Không thể dịch văn bản này",
        }
    }
}

pub async fn translate(
    text: &str,
    max_chars: usize,
    target_lang: &str,
    timeout_secs: u64,
) -> Result<String, TranslationError> {
    client::call_translate_api(text, max_chars, target_lang, timeout_secs).await
}
