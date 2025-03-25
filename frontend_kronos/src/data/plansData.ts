import { Plan } from "../types/Plan";

const plansData: Plan[] = [
  {
    id: 1,
    type: "PLAN",
    number: "25-00",
    date: "01JAN2025",
    published: "03JAN2025",
    expires: "31DEC2025",
    taskedBy: "101st BDE",
    taskedTo: "A, B, C, D, E, F",
    mission: "Ensure readiness for operational deployment."
  },
  {
    id: 2,
    type: "WARNO",
    number: "25-00-00",
    date: "02JAN2025",
    published: "04JAN2025",
    expires: "10JAN2025",
    taskedBy: "101st BDE",
    taskedTo: "A, C",
    mission: "Initial warning for upcoming deployment activities."
  },
  {
    id: 3,
    type: "OPORD",
    number: "25-00-01",
    date: "05JAN2025",
    published: "06JAN2025",
    expires: "31DEC2025",
    taskedBy: "101st BDE",
    taskedTo: "A, B, C, D, E, F",
    mission: "Operational order for deployment and sustainment ops."
  },
  {
    id: 4,
    type: "FRAGO",
    number: "25-00-02",
    date: "15JAN2025",
    published: "16JAN2025",
    expires: "30JAN2025",
    taskedBy: "101st BDE",
    taskedTo: "B, D",
    mission: "Change to logistics schedule for Phase 1."
  },
  {
    id: 5,
    type: "PLAN",
    number: "25-01",
    date: "01FEB2025",
    published: "03FEB2025",
    expires: "30JUN2025",
    taskedBy: "102nd BDE",
    taskedTo: "A, B, D",
    mission: "Phase 2 readiness and mobilization."
  },
  {
    id: 6,
    type: "FRAGO",
    number: "25-01-01",
    date: "10FEB2025",
    published: "11FEB2025",
    expires: "20FEB2025",
    taskedBy: "102nd BDE",
    taskedTo: "A, B",
    mission: "Updated unit training timeline for Phase 2."
  }
];

export default plansData;
