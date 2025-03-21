import React from "react";
import { Plan } from "../types/planTypes";

interface PlanCardProps {
  plan: Plan;
  selectPlan: (plan: Plan) => void;
}

const PlanCard: React.FC<PlanCardProps> = ({ plan, selectPlan }) => {
  return (
    <div className="plan-card" onClick={() => selectPlan(plan)}>
      {/* Left Section */}
      <div className="plan-left">
        <span className={`plan-type ${plan.type.toLowerCase()}`}>
          {plan.type}
        </span>
        <h1 className="plan-title">{plan.number}</h1>
        <h2 className="plan-subtitle">FY 2025 Base Order</h2>
      </div>

      {/* Right Section */}
      <div className="plan-right">
        <p className="plan-info"><strong>Published:</strong> {plan.published}</p>
        <p className="plan-info"><strong>Expires:</strong> {plan.expires}</p>
        <p className="plan-info"><strong>Tasked By:</strong> {plan.taskedBy}</p>
        <p className="plan-info"><strong>Tasked To:</strong> {plan.taskedTo}</p>
        <p className="plan-info"><strong>Mission:</strong> {plan.mission}</p>
      </div>
    </div>
  );
};

export default PlanCard;
