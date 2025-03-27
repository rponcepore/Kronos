/*
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

*/

import { Classification } from "../enums/Classification";

export type Plan = {

    id: number,
    unit: String,
    parent_plan: number | null, // can be null,
    fiscal_year: number,
    serial_number: number,
    classification: Classification,
    name: string,

}

