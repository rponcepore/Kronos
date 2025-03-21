import React, { useState } from "react";
import PlansList from "../components/PlansList";
import PlanDetails from "../components/PlansDetails";
import { Plan } from "../types/planTypes";

const plansData: Plan[] = [
  { id: 1, type: "OPORD", number: "25-00", date: "15 FEB 2024", published: "03JAN2025", expires: "31DEC2025", taskedBy: "101st BDE", taskedTo: "A, B, C, D, E, F", mission: "Ensure readiness for operational deployment." },
  { id: 2, type: "FRAGO", number: "25-01", date: "16 FEB 2024", published: "05JAN2025", expires: "30MAR2025", taskedBy: "102nd BDE", taskedTo: "A, B, C", mission: "Modify Phase 2 logistics support." },
  { id: 3, type: "WARNO", number: "25-02", date: "17 FEB 2024", published: "10JAN2025", expires: "15APR2025", taskedBy: "103rd BDE", taskedTo: "B, C, D", mission: "Prepare for large-scale training exercise." }
];

const PlansPage: React.FC = () => {
  const [selectedPlan, setSelectedPlan] = useState<Plan | null>(null);

  return (
    <div className="p-6 mt-16">
      {selectedPlan ? (
        <PlanDetails plan={selectedPlan} goBack={() => setSelectedPlan(null)} />
      ) : (
        <PlansList plans={plansData} selectPlan={setSelectedPlan} />
      )}
    </div>
  );
};

export default PlansPage;
