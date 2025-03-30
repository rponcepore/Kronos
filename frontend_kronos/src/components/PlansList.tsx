// Importing React for JSX usage
import React from "react";

// Importing the PlanSummary type definition
import { PlanSummary } from "../types/frontend_types/PlanSummary";

// Importing the reusable PlanCard component
import PlanCard from "./PlanCard";

// Defining the props expected by the PlansList component
interface PlansListProps {
  plans: PlanSummary[];                       // Array of PlanSummary objects to display
  selectPlan: (plan: PlanSummary) => void;   // Callback to handle clicking a plan
}

// Functional component to display a list/grid of PlanCard components
const PlansList: React.FC<PlansListProps> = ({ plans, selectPlan }) => {
  return (
    // Top-level wrapper with layout/padding styles
    <div className="plans-overview">
      {/* Flex or grid container for laying out individual plan cards */}
      <div className="plans-grid">
        {/* Map through each plan and render a PlanCard component */}
        {plans.map((plan) => (
          <PlanCard 
            key={plan.data.id}             // Unique key for React's rendering
            plan={plan}               // Passing down the plan data
            selectPlan={selectPlan}   // Passing down the click handler
          />
        ))}
      </div>
    </div>
  );
};

// Exporting the component so it can be used in other files
export default PlansList;

