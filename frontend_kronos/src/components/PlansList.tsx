import React from "react";
import PlanCard from "./PlanCard";
import { Plan } from "../types/planTypes";

interface PlansListProps {
  plans: Plan[];
  selectPlan: (plan: Plan) => void;
}

const PlansList: React.FC<PlansListProps> = ({ plans, selectPlan }) => {
  return (
    <div className="plans-container">
      {plans.map((plan) => (
        <div className="mb-6"> 
        <PlanCard key={plan.id} plan={plan} selectPlan={selectPlan} />
        </div>
      ))}
    </div>
  );
};

export default PlansList;
