import React from "react";
import { PlanSummary } from "../../types/frontend_types/PlanSummary";                                // Import the PlanSummary type
import { getPlanSerialDisplay } from "../../helper_methods/format";     // Utility to format serial numbers

// ------------------------------
// Props interface for PlanCard
// ------------------------------
interface PlanCardProps {
  plan: PlanSummary;                                     // The plan object to render
  selectPlan: (plan: PlanSummary) => void;               // Function to call when the card is clicked
}

// ------------------------------
// PlanCard Component
// ------------------------------
const PlanCard: React.FC<PlanCardProps> = ({ plan, selectPlan }) => {
  return (
    // When clicked, call selectPlan with this specific plan object
    <div className="plan-card" onClick={() => selectPlan(plan)}>
      {/* Plan title */}
      <h2>{plan.data.name}</h2>

      {/* Subtitle: shows fiscal year */}
      <p className="plan-subtitle">FY {plan.data.fiscal_year} Base Plan</p>

      {/* Display key metadata about the plan */}
      <p><strong>UIC:</strong> {plan.data.uic}</p>
      <p><strong>Classification:</strong> {plan.data.classification}</p>

      {/* Serial number formatted for display */}
      <p><strong>Serial #:</strong> {getPlanSerialDisplay(plan.data.fiscal_year, plan.data.serial_number)}</p>
    </div>
  );
};

export default PlanCard;







