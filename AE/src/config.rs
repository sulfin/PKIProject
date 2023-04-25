use std::{fs, io};
use std::io::Result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AEDatabase {
    csrs: Vec<CSRDatabase>,
    crts: Vec<CRTDatabase>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CSRDatabase {
    pub email: String,
    pub csr_path: String,
    pub otp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CRTDatabase {
    pub email: String,
    pub crt_id: String,
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

    pub fn get_csr(&self, email: &str) -> Option<&CSRDatabase> {
        self.csrs.iter().find(|csr| csr.email == email)
    }
    pub fn remove_csr(&mut self, email: &str) -> Result<()> {
        self.csrs.retain(|csr| csr.email != email);
        self.save()?;
        Ok(())
    }

    pub fn add_crt(&mut self, crt: &CRTDatabase) -> Result<()> {
        self.crts.push(crt.to_owned());
        self.save()?;
        Ok(())
    }
    pub fn get_crt(&self, email: &str) -> Option<&CRTDatabase> {
        self.crts.iter().find(|crt| crt.email == email)
    }
    pub fn get_crt_by_id(&self, id: &str) -> Option<&CRTDatabase> {
        self.crts.iter().find(|crt| crt.crt_id == id)
    }
    pub fn remove_crt(&mut self, email: &str) -> Result<()> {
        self.crts.retain(|crt| crt.email != email);
        self.save()?;
        Ok(())
    }
}