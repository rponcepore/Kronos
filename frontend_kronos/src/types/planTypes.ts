export interface Plan {
  id: number;
  unit: string;
  parent_plan: number | null;
  fiscal_year: number;
  serial_number: number;
  classification: string;
  name: string;
  
  // Frontend-only derived fields for display
  number: string;
  date: string;
  published: string;
  expires: string;
  type: string;
  mission: string;
  taskedBy: string;
  taskedTo: string;
};