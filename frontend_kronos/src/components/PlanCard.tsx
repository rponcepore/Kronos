import React from "react";
import { Plan } from "../types/Plan";

interface PlanCardProps {
  plan: Plan;
  selectPlan: (plan: Plan) => void;
}

const PlanCard: React.FC<PlanCardProps> = ({ plan, selectPlan }) => {
  return (
    <div className="plan-card" onClick={() => selectPlan(plan)}>
      <div className="plan-left">
        <h2 className="plan-title">{plan.name}</h2>
        <p className="plan-subtitle">FY {plan.fiscal_year} Base Plan</p>
      </div>
      <div className="plan-right">
        <p className="plan-info"><strong>Unit:</strong> {plan.unit}</p>
        <p className="plan-info"><strong>Classification:</strong> {plan.classification}</p>
        <p className="plan-info"><strong>Serial #:</strong> {plan.serial_number}</p>
      </div>
    </div>
  );
};

export default PlanCard;




