use std::{fs, io};
use std::io::Result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AEDatabase {
    csrs: Vec<CSRDatabase>,
    crts: Vec<CRTDatabase>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CSRDatabase {
    pub email: String,
    pub csr_path: String,
    pub otp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CRTDatabase {
    pub email: String,
    pub crt_path: String,
    pub otp_revoc: String,
}

impl AEDatabase {
    fn new() -> Self {
        AEDatabase {
            csrs: Vec::new(),
            crts: Vec::new(),
        }
    }

    pub fn get() -> Result<Self> {
        if fs::metadata("./config.json").is_err() {
            let db = AEDatabase::new();
            db.save()?;
            return Ok(db);
        }
        Ok(serde_json::from_str(&fs::read_to_string("./config.json")?)?)
    }

    fn save(&self) -> Result<()> {
        fs::write("./config.json", serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn add_csr(&mut self, csr: CSRDatabase) -> Result<()> {
        self.csrs.push(csr);
        self.save()?;
        Ok(())
    }

}