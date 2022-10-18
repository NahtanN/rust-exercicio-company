use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, EnumCountMacro, EnumIter)]
pub enum CompanyOptions {
    AddEmployee,
    AllInDepartment,
    AllInCompany,
    Close,
}

pub struct Choice {
    pub id: u16,
    pub description: String,
}

impl CompanyOptions {
    pub fn count_options() -> u16 {
        CompanyOptions::COUNT as u16
    }

    pub fn data(&self) -> Choice {
        match self {
            Self::AddEmployee => Choice {
                id: 1,
                description: "Add employee".to_string(),
            },
            Self::AllInDepartment => Choice {
                id: 2,
                description: "Get all people in a department".to_string(),
            },
            Self::AllInCompany => Choice {
                id: 3,
                description: "Get all people in the company".to_string(),
            },
            Self::Close => Choice {
                id: 4,
                description: "Close".to_string(),
            },
        }
    }
}
