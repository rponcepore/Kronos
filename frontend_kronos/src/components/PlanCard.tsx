import React from "react";
import { Plan } from "../types/Plan";                                // Import the Plan type
import { getPlanSerialDisplay } from "../helper_methods/format";     // Utility to format serial numbers

// ------------------------------
// Props interface for PlanCard
// ------------------------------
interface PlanCardProps {
  plan: Plan;                                     // The plan object to render
  selectPlan: (plan: Plan) => void;               // Function to call when the card is clicked
}

// ------------------------------
// PlanCard Component
// ------------------------------
const PlanCard: React.FC<PlanCardProps> = ({ plan, selectPlan }) => {
  return (
    // When clicked, call selectPlan with this specific plan object
    <div className="plan-card" onClick={() => selectPlan(plan)}>
      {/* Plan title */}
      <h2>{plan.name}</h2>

      {/* Subtitle: shows fiscal year */}
      <p className="plan-subtitle">FY {plan.fiscal_year} Base Plan</p>

      {/* Display key metadata about the plan */}
      <p><strong>Unit:</strong> {plan.unit}</p>
      <p><strong>Classification:</strong> {plan.classification}</p>

      {/* Serial number formatted for display */}
      <p><strong>Serial #:</strong> {getPlanSerialDisplay(plan.fiscal_year, plan.serial_number)}</p>
    </div>
  );
};

export default PlanCard;







