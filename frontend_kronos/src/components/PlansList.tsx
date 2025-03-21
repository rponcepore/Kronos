import React from "react";
import PlanCard from "./PlanCard";
import { Plan } from "../types/planTypes";

interface PlansListProps {
  plans: Plan[];
  selectPlan: (plan: Plan) => void;
}

const PlansList: React.FC<PlansListProps> = ({ plans, selectPlan }) => {
  return (
    <div className="p-6 space-y-4">
      {plans.map((plan) => (
        <PlanCard key={plan.id} plan={plan} selectPlan={selectPlan} />
      ))}
    </div>
  );
};

export default PlansList;
