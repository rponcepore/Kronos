//! army_echelons_enum.rs

// Echelons for units in the US Army
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Echelon {
    TM,
    SQD,
    SEC,
    PLT,
    DET,
    CO,
    BN,
    BDE,
    DIV,
    CORPS,
    MACOM,
    CNTR,
    ARMY,
    DRU,
    ASCC,
    ACOM,
    HQ,
    MILSVC,
    STATE,
    FACILITY,
    MSC,
    CONTAINER,
    UNK,
}

impl Echelon {
    pub fn as_str(&self) -> &'static str {
        match self {
            Echelon::TM => "TM",
            Echelon::SQD => "SQD",
            Echelon::SEC => "SEC",
            Echelon::PLT => "PLT",
            Echelon::DET => "DET",
            Echelon::CO => "CO",
            Echelon::BN => "BN",
            Echelon::BDE => "BDE",
            Echelon::DIV => "DIV",
            Echelon::CORPS => "CORPS",
            Echelon::MACOM => "MACOM",
            Echelon::CNTR => "CNTR",
            Echelon::ARMY => "ARMY",
            Echelon::DRU => "DRU",
            Echelon::ASCC => "ASCC",
            Echelon::ACOM => "ACOM",
            Echelon::HQ => "HQ",
            Echelon::MILSVC => "MILSVC",
            Echelon::STATE => "STATE",
            Echelon::FACILITY => "FACILITY",
            Echelon::MSC => "MSC",
            Echelon::CONTAINER => "CONTAINER",
            Echelon::UNK => "UNK",
        }
    }

    pub fn as_long_str(&self) -> &'static str {
        match self {
            Echelon::TM => "Team",
            Echelon::SQD => "Squad",
            Echelon::SEC => "Section",
            Echelon::PLT => "Platoon",
            Echelon::DET => "Detachment",
            Echelon::CO => "Company",
            Echelon::BN => "Battalion",
            Echelon::BDE => "Brigade",
            Echelon::DIV => "Division",
            Echelon::CORPS => "Corps",
            Echelon::MACOM => "Major Command",
            Echelon::CNTR => "Center",
            Echelon::ARMY => "Army",
            Echelon::DRU => "Direct Reporting Unit",
            Echelon::ASCC => "Army Service Component Command",
            Echelon::ACOM => "Army Command",
            Echelon::HQ => "Headquarters",
            Echelon::MILSVC => "Military Service",
            Echelon::STATE => "State Army National Guard",
            Echelon::FACILITY => "Aviation Support Facility",
            Echelon::MSC => "Major Subordinate Command",
            Echelon::CONTAINER => "Container for Organizing Units",
            Echelon::UNK => "Unknown",
        }
    }

    fn from_str(input: &str) -> Result<Self, &str> {
        match input {
            "TM" | "Team" => Ok(Echelon::TM),
            "SQD" | "Squad" => Ok(Echelon::SQD),
            "SEC" | "Section" => Ok(Echelon::SEC),
            "PLT" | "Platoon" => Ok(Echelon::PLT),
            "DET" | "Detachment" => Ok(Echelon::DET),
            "CO" | "Company" => Ok(Echelon::CO),
            "BN" | "Battalion" => Ok(Echelon::BN),
            "BDE" | "Brigade" => Ok(Echelon::BDE),
            "DIV" | "Division" => Ok(Echelon::DIV),
            "CORPS" | "Corps" => Ok(Echelon::CORPS),
            "MACOM" | "Major Command" => Ok(Echelon::MACOM),
            "CNTR" | "Center" => Ok(Echelon::CNTR),
            "ARMY" | "Army" => Ok(Echelon::ARMY),
            "DRU" | "Direct Reporting Unit" => Ok(Echelon::DRU),
            "ASCC" | "Army Service Component Command" => Ok(Echelon::ASCC),
            "ACOM" | "Army Command" => Ok(Echelon::ACOM),
            "HQ" | "Headquarters" => Ok(Echelon::HQ),
            "MILSVC" | "Military Service" => Ok(Echelon::MILSVC),
            "STATE" | "State Army National Guard" => Ok(Echelon::STATE),
            "FACILITY" | "Aviation Support Facility" => Ok(Echelon::FACILITY),
            "MSC" | "Major Subordinate Command" => Ok(Echelon::MSC),
            "CONTAINER" | "Container for Organizing Units" => Ok(Echelon::CONTAINER),
            "UNK" | "Unknown" => Ok(Echelon::UNK),
            _ => Err("Unknown Echelon supplied. Supply as an all-caps echelon or full name, e.g., 'DIV' or 'Division'."),
        }
    }
}
