import React from "react";
import { Plan } from "../types/planTypes";

interface PlanDetailsProps {
  plan: Plan;
  goBack: () => void;
}

const PlanDetails: React.FC<PlanDetailsProps> = ({ plan, goBack }) => {
  return (
    <div className="plan-details">
      <button onClick={goBack} className="back-button">← Back</button>

      <span className={`plan-type ${plan.type.toLowerCase()}`}>
        {plan.type}
      </span>

      <h1 className="details-title">{plan.number}</h1>
      <p className="details-info"><strong>Published:</strong> {plan.published}</p>
      <p className="details-info"><strong>Expires:</strong> {plan.expires}</p>
      <p className="details-info"><strong>Tasked By:</strong> {plan.taskedBy}</p>
      <p className="details-info"><strong>Tasked To:</strong> {plan.taskedTo}</p>
      <p className="details-info"><strong>Mission:</strong> {plan.mission}</p>
    </div>
  );
};

export default PlanDetails;
