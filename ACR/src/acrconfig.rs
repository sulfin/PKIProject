
#[derive(Debug, Clone, Default)]
pub struct AcrConfig {
    pub valid_time: u32,
    pub path_len: u32,
    pub country: String,
    pub common_name: String,
}

impl AcrConfig {
    pub fn new(valid_time: u32,path_len: u32 ,country: String, common_name: String) -> Self {
        AcrConfig {
            valid_time,
            path_len,
            country,
            common_name,
        }
    }

    pub fn default() -> Self {
        AcrConfig {
            valid_time: 365,
            path_len: 3,
            country: "FR".to_string(),
            common_name: "Pas Un Virus Sign".to_string(),
        }
    }
}