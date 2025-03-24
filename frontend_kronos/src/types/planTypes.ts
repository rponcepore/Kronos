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
  