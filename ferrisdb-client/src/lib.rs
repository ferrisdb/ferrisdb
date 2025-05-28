// FerrisDB client library
pub struct FerrisDB;

impl FerrisDB {
    pub async fn connect(_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self)
    }
}
