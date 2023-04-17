
#[derive(Debug, Clone, Default)]
pub struct AcrConfig {
    pub valid_time: u32,
    pub path_len: u32,
    pub country: String,
    pub common_name: String,
}

impl AcrConfig {

    pub fn default() -> Self {
        AcrConfig {
            valid_time: 365*25,
            path_len: 1,
            country: "FR".to_string(),
            common_name: "Pas Un Virus Sign".to_string(),
        }
    }
}