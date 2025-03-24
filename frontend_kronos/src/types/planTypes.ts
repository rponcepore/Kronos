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
  
  // Function to serialize 
  export const serializeToPlan = (json: any): Plan => {
    return {
      id: json.id,
      type: json.type,
      number: json.number,
      date: json.date,
      published: json.published,
      expires: json.expires,
      taskedBy: json.taskedBy,
      taskedTo: json.taskedTo,
      mission: json.mission
    };
  };
  