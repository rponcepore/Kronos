export interface Plan {
    id: number;
    type: "OPORD" | "FRAGO" | "WARNO";
    number: string;
    date: string;
  }
  