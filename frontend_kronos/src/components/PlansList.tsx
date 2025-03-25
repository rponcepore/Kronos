import React from "react";
import PlanCard from "./PlanCard";
import { Plan } from "../types/Plan";

interface PlansListProps {
  plans: Plan[];
  selectPlan: (plan: Plan) => void;
  hideTypeBadge?: boolean;
}

const PlansList: React.FC<PlansListProps> = ({
  plans,
  selectPlan,
  hideTypeBadge = false
}) => {
  return (
    <div className="plans-container">
      {plans.map((plan) => (
        <div key={plan.id} className="mb-6">
          <PlanCard
            plan={plan}
            selectPlan={selectPlan}
            hideTypeBadge={hideTypeBadge}
          />
        </div>
      ))}
    </div>
  );
};


export default PlansList; 