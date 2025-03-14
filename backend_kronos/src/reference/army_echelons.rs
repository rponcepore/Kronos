//! army_echelons.rs

use sea_orm::DeriveIden;
use sea_orm_migration::prelude::{sea_query::extension::postgres::Type, *};

// Echelons for units in the US Army
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveIden)]
pub enum Echelon {
    #[sea_orm(iden = "echelon")]
    Enum,
    #[sea_orm(iden = "Team")]
    Team,
    #[sea_orm(iden = "Squad")]
    Squad,
    #[sea_orm(iden = "Section")]
    Section,
    /*
    Platoon,
    Detachment,
    Company,
    Battalion,
    Brigade,
    Division,
    Corps,
    Macom,
    Center,
    Army,
    Dru,
    Ascc,
    Acom,
    Headquarters,
    Department,
    StateGuard,
    Facility,
    Msc,
    Container,
    Unknown,
     */
}

/* 
impl Echelon {
    /// Returns the short code for each echelon.
    pub fn as_str(&self) -> &'static str {
        match self {
            Echelon::Team => "TM",
            Echelon::Squad => "SQD",
            Echelon::Section => "SEC",
            Echelon::Platoon => "PLT",
            Echelon::Detachment => "DET",
            Echelon::Company => "CO",
            Echelon::Battalion => "BN",
            Echelon::Brigade => "BDE",
            Echelon::Division => "DIV",
            Echelon::Corps => "CORPS",
            Echelon::Macom => "MACOM",
            Echelon::Center => "CNTR",
            Echelon::Army => "ARMY",
            Echelon::Dru => "DRU",
            Echelon::Ascc => "ASCC",
            Echelon::Acom => "ACOM",
            Echelon::Headquarters => "HQ",
            Echelon::Department => "MILSVC",
            Echelon::StateGuard => "STATE",
            Echelon::Facility => "FACILITY",
            Echelon::Msc => "MSC",
            Echelon::Container => "CONTAINER",
            Echelon::Unknown => "UNK",
        }
    }

    pub fn as_long_str(&self) -> &'static str {
        match self {
            Echelon::Team => "Team",
            Echelon::Squad => "Squad",
            Echelon::Section => "Section",
            Echelon::Platoon => "Platoon",
            Echelon::Detachment => "Detachment",
            Echelon::Company => "Company",
            Echelon::Battalion => "Battalion",
            Echelon::Brigade => "Brigade",
            Echelon::Division => "Division",
            Echelon::Corps => "Corps",
            Echelon::Macom => "Major Command",
            Echelon::Center => "Center",
            Echelon::Army => "Army",
            Echelon::Dru => "Direct Reporting Unit",
            Echelon::Ascc => "Army Service Component Command",
            Echelon::Acom => "Army Command",
            Echelon::Headquarters => "Headquarters",
            Echelon::Department => "Military Service",
            Echelon::StateGuard => "State Army National Guard",
            Echelon::Facility => "Aviation Support Facility",
            Echelon::Msc => "Major Subordinate Command",
            Echelon::Container => "Container for Organizing Units",
            Echelon::Unknown => "Unknown",
        }
    }    

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "TM" => Ok(Echelon::Team),
            "SQD" => Ok(Echelon::Squad),
            "SEC" => Ok(Echelon::Section),
            "PLT" => Ok(Echelon::Platoon),
            "DET" => Ok(Echelon::Detachment),
            "CO" => Ok(Echelon::Company),
            "BN" => Ok(Echelon::Battalion),
            "BDE" => Ok(Echelon::Brigade),
            "DIV" => Ok(Echelon::Division),
            "CORPS" => Ok(Echelon::Corps),
            "MACOM" => Ok(Echelon::Macom),
            "CNTR" => Ok(Echelon::Center),
            "ARMY" => Ok(Echelon::Army),
            "DRU" => Ok(Echelon::Dru),
            "ASCC" => Ok(Echelon::Ascc),
            "ACOM" => Ok(Echelon::Acom),
            "HQ" => Ok(Echelon::Headquarters),
            "MILSVC" => Ok(Echelon::Department),
            "STATE" => Ok(Echelon::StateGuard),
            "FACILITY" => Ok(Echelon::Facility),
            "MSC" => Ok(Echelon::Msc),
            "CONTAINER" => Ok(Echelon::Container),
            "UNK" => Ok(Echelon::Unknown),
            _ => Err("Unknown Echelon supplied. Supply as a all caps ecehlon, i.e., DIV."),
        }
    }
    
}
*/