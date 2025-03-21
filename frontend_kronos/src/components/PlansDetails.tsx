import React from "react";
import { Plan } from "../types/planTypes";

interface PlanDetailsProps {
  plan: Plan;
  goBack: () => void;
}

const PlanDetails: React.FC<PlanDetailsProps> = ({ plan, goBack }) => {
  return (
    <div className="p-4 bg-gray-900 text-white min-h-screen">
      <button onClick={goBack} className="mb-4 text-blue-400">
        &larr; Back
      </button>
      <h1 className="text-2xl font-bold mb-2">
        {plan.type} {plan.number}
      </h1>
      <p className="text-gray-400">Date: {plan.date}</p>
      <div className="mt-4 bg-gray-800 p-4 rounded-lg">
        <p className="text-gray-300">
          Placeholder content for {plan.type} details.
        </p>
      </div>
    </div>
  );
};

export default PlanDetails;
