import React from "react";
import { Plan } from "../types/Plan";

interface PlanDetailsProps {
  plan: Plan;
  allPlans: Plan[];
  goBack: () => void;
}

const PlanDetails: React.FC<PlanDetailsProps> = ({ plan, allPlans, goBack }) => {
  const childPlans = allPlans.filter(
    (p) => p.number.startsWith(plan.number) && p.id !== plan.id
  );

  return (
    <div className="plan-details">
      {/* Back Button Aligned Left */}
      <div className="back-button-wrapper">
        <button className="back-button" onClick={goBack}>← Back</button>
      </div>

      {/* Main Plan Info */}
      <h1>
        <span className={`plan-type ${plan.type.toLowerCase()}`}>{plan.type}</span> {plan.number}
      </h1>

      <p><strong>Published:</strong> {plan.published}</p>
      <p><strong>Expires:</strong> {plan.expires}</p>
      <p><strong>Tasked By:</strong> {plan.taskedBy}</p>
      <p><strong>Tasked To:</strong> {plan.taskedTo}</p>
      <p><strong>Mission:</strong> {plan.mission}</p>

      {/* Related Orders Section */}
      {childPlans.length > 0 && (
  <div className="related-orders">
    <h2>Related Orders</h2>
    <div className="related-orders-container">
      {childPlans.map((child) => (
        <div key={child.id} className="plan-card">
          <span className={`plan-type ${child.type.toLowerCase()}`}>{child.type}</span>
          <h3>{child.number}</h3>
          <p><strong>Published:</strong> {child.published}</p>
          <p><strong>Expires:</strong> {child.expires}</p>
          <p><strong>Tasked To:</strong> {child.taskedTo}</p>
          <p><strong>Mission:</strong> {child.mission}</p>
        </div>
      ))}
    </div>
  </div>
)}

    </div>
  );
};

export default PlanDetails;



