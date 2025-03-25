import React from "react";
import { Plan } from "../types/Plan";

interface PlanCardProps {
  plan: Plan;
  selectPlan: (plan: Plan) => void;
  hideTypeBadge?: boolean;
}

const PlanCard: React.FC<PlanCardProps> = ({ plan, selectPlan }) => {
  return (
    <div className="plan-card" onClick={() => selectPlan(plan)}>
      <div className="plan-left">
        <h2 className="plan-title">Plan {plan.number}</h2>
        <p className="plan-subtitle">FY {plan.date.slice(2, 6)} Base Order</p>
      </div>
      <div className="plan-right">
        <p className="plan-info"><strong>Published:</strong> {plan.published}</p>
        <p className="plan-info"><strong>Expires:</strong> {plan.expires}</p>
        <p className="plan-info"><strong>Tasked By:</strong> {plan.taskedBy}</p>
        <p className="plan-info"><strong>Mission:</strong> {plan.mission}</p>
      </div>
    </div>
  );
};

export default PlanCard;



