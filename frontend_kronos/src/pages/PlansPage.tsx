import React, { useState } from "react";
import PlansList from "../components/PlansList";
import PlanDetails from "../components/PlansDetails";
import { Plan } from "../types/planTypes";

const plansData: Plan[] = [
  { id: 1, type: "OPORD", number: "25-00", date: "15 FEB 2024" },
  { id: 2, type: "FRAGO", number: "25-01", date: "16 FEB 2024" },
  { id: 3, type: "FRAGO", number: "25-02", date: "17 FEB 2024" },
  { id: 4, type: "WARNO", number: "25-03", date: "18 FEB 2024" },
];

const PlansPage: React.FC = () => {
  const [selectedPlan, setSelectedPlan] = useState<Plan | null>(null);

  return (
    <div>
      {selectedPlan ? (
        <PlanDetails plan={selectedPlan} goBack={() => setSelectedPlan(null)} />
      ) : (
        <PlansList plans={plansData} selectPlan={setSelectedPlan} />
      )}
    </div>
  );
};

export default PlansPage;
