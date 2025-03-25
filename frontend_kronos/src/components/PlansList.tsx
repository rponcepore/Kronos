import React from "react";
import PlanCard from "./PlanCard";
import { Plan } from "../types/Plan";

interface PlansListProps {
  plans: Plan[];
  selectPlan: (plan: Plan) => void;
}

const PlansList: React.FC<PlansListProps> = ({ plans, selectPlan }) => {
  return (
    <div className="plans-container">
      {plans.map((plan) => (
        <div key={plan.id} className="mb-6">
          <PlanCard plan={plan} selectPlan={selectPlan} />
        </div>
      ))}
    </div>
  );
};

export default PlansList;
