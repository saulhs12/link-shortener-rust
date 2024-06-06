use base64::engine::general_purpose;
use base64::Engine;
use rand::Rng;

pub trait GenerateLink {
    fn generate_id() -> String;
}
pub struct GenerateLinkService {}

impl GenerateLink for GenerateLinkService {
    fn generate_id() -> String {
        let random_number = rand::thread_rng().gen_range(0..u32::MAX);
        general_purpose::URL_SAFE_NO_PAD.encode(random_number.to_string())
    }
}
