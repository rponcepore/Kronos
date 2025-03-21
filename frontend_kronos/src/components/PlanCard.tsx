import React from "react";
import { Plan } from "../types/planTypes";

interface PlanCardProps {
  plan: Plan;
  selectPlan: (plan: Plan) => void;
}

const PlanCard: React.FC<PlanCardProps> = ({ plan, selectPlan }) => {
  return (
    <div
      className="w-full bg-white rounded-2xl shadow-md p-6 cursor-pointer hover:shadow-lg transition"
      onClick={() => selectPlan(plan)}
    >
      {/* Plan Type Label */}
      <span className={`text-sm font-bold px-3 py-1 rounded-full ${
        plan.type === "OPORD" ? "bg-blue-500 text-white" :
        plan.type === "FRAGO" ? "bg-yellow-500 text-black" :
        "bg-red-500 text-white"
      }`}>
        {plan.type}
      </span>

      {/* Plan Number & Title */}
      <h1 className="text-3xl font-semibold mt-2">{plan.number}</h1>
      <h2 className="text-xl text-gray-600">{plan.mission}</h2>

      {/* Date Info */}
      <div className="text-gray-500 mt-3 text-sm">
        <p><strong>Published:</strong> {plan.published}</p>
        <p><strong>Expires:</strong> {plan.expires}</p>
      </div>
    </div>
  );
};

export default PlanCard;
