export interface Plan {
    id: number;
    type: "PLAN" | "OPORD" | "FRAGO" | "WARNO";
    number: string;
    date: string;
    published: string;
    expires: string;
    taskedBy: string;
    taskedTo: string;
    mission: string;
  }

/*

export type Plan = {

    id: number,
    unit: String,
    parent_plan: number, // can be null,
    fiscal_year: number,
    serial_number: number,
    classification: string,
    name: string,

}

*/