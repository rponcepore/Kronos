//! Echelon.tsx

// armyEchelons.ts

export enum Echelon {
    TM = "TM",
    SQD = "SQD",
    SEC = "SEC",
    PLT = "PLT",
    DET = "DET",
    CO = "CO",
    BN = "BN",
    BDE = "BDE",
    DIV = "DIV",
    CORPS = "CORPS",
    MACOM = "MACOM",
    CNTR = "CNTR",
    ARMY = "ARMY",
    DRU = "DRU",
    ASCC = "ASCC",
    ACOM = "ACOM",
    HQ = "HQ",
    MILSVC = "MILSVC",
    STATE = "STATE",
    FACILITY = "FACILITY",
    MSC = "MSC",
    CONTAINER = "CONTAINER",
    UNK = "UNK",
}

export function echelonToString(echelon: Echelon): string {
    switch (echelon) {
        case Echelon.TM: return "Team";
        case Echelon.SQD: return "Squad";
        case Echelon.SEC: return "Section";
        case Echelon.PLT: return "Platoon";
        case Echelon.DET: return "Detachment";
        case Echelon.CO: return "Company";
        case Echelon.BN: return "Battalion";
        case Echelon.BDE: return "Brigade";
        case Echelon.DIV: return "Division";
        case Echelon.CORPS: return "Corps";
        case Echelon.MACOM: return "Major Command";
        case Echelon.CNTR: return "Center";
        case Echelon.ARMY: return "Army";
        case Echelon.DRU: return "Direct Reporting Unit";
        case Echelon.ASCC: return "Army Service Component Command";
        case Echelon.ACOM: return "Army Command";
        case Echelon.HQ: return "Headquarters";
        case Echelon.MILSVC: return "Military Service";
        case Echelon.STATE: return "State Army National Guard";
        case Echelon.FACILITY: return "Aviation Support Facility";
        case Echelon.MSC: return "Major Subordinate Command";
        case Echelon.CONTAINER: return "Container for Organizing Units";
        case Echelon.UNK: return "Unknown";
        default: return "Unknown";
    }
}

export function stringToEchelon(input: string): Echelon | null {
    switch (input.toUpperCase()) {
        case "TM":
        case "TEAM":
            return Echelon.TM;
        case "SQD":
        case "SQUAD":
            return Echelon.SQD;
        case "SEC":
        case "SECTION":
            return Echelon.SEC;
        case "PLT":
        case "PLATOON":
            return Echelon.PLT;
        case "DET":
        case "DETACHMENT":
            return Echelon.DET;
        case "CO":
        case "COMPANY":
            return Echelon.CO;
        case "BN":
        case "BATTALION":
            return Echelon.BN;
        case "BDE":
        case "BRIGADE":
            return Echelon.BDE;
        case "DIV":
        case "DIVISION":
            return Echelon.DIV;
        case "CORPS":
        // case "CORPS": // Duplicate because no long name
            return Echelon.CORPS;
        case "MACOM":
        case "MAJOR COMMAND":
            return Echelon.MACOM;
        case "CNTR":
        case "CENTER":
            return Echelon.CNTR;
        case "ARMY":
        // case "ARMY": // Duplicate because no long name
            return Echelon.ARMY;
        case "DRU":
        case "DIRECT REPORTIGN UNIT":
            return Echelon.DRU;
        case "ASCC":
        case "ARMY SERVICE COMPONENT COMMAND":
            return Echelon.ASCC;
        case "ACOM":
        case "ARMY COMMAND":
            return Echelon.ACOM;
        case "HQ":
        case "HEADQUARTERS":
            return Echelon.HQ;
        case "MILSVC":
        case "MILITARY SERVICE":
            return Echelon.MILSVC;
        case "STATE":
        case "STATE ARMY NATIONAL GUARD":
            return Echelon.STATE;
        case "FACILITY":
        case "AVIATION SUPPORTING FACILITY":
            return Echelon.FACILITY;
        case "MSC":
        case "MAJOR SUBORDINATE COMMAND":
            return Echelon.MSC;
        case "CONTAINER":
        case "CONTAINER FOR ORGANIZING UNITS":
            return Echelon.CONTAINER;
        case "UNK":
        case "UNKNOWN":
            return Echelon.UNK;
        default:
            return null;
    }
}