import React from "react";
import { Plan } from "../types/planTypes";

interface PlanDetailsProps {
  plan: Plan;
  goBack: () => void;
}

const PlanDetails: React.FC<PlanDetailsProps> = ({ plan, goBack }) => {
  return (
    <div className="p-6 bg-white rounded-2xl shadow-md max-w-4xl mx-auto">
      <button onClick={goBack} className="text-blue-500 mb-4">
        ← Back
      </button>

      <span className={`text-sm font-bold px-3 py-1 rounded-full ${
        plan.type === "OPORD" ? "bg-blue-500 text-white" :
        plan.type === "FRAGO" ? "bg-yellow-500 text-black" :
        "bg-red-500 text-white"
      }`}>
        {plan.type}
      </span>

      <h1 className="text-3xl font-bold mt-2">{plan.number}</h1>
      <p className="text-gray-600 text-lg">{plan.mission}</p>

      <div className="mt-4 text-gray-700">
        <p><strong>Published:</strong> {plan.published}</p>
        <p><strong>Expires:</strong> {plan.expires}</p>
        <p><strong>Tasked By:</strong> {plan.taskedBy}</p>
        <p><strong>Tasked To:</strong> {plan.taskedTo}</p>
      </div>
    </div>
  );
};

export default PlanDetails;
