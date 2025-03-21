import React from "react";
import { Plan } from "../types/planTypes";

interface PlansListProps {
  plans: Plan[];
  selectPlan: (plan: Plan) => void;
}

const PlansList: React.FC<PlansListProps> = ({ plans, selectPlan }) => {
  return (
    <div className="p-4 bg-gray-900 text-white min-h-screen">
      <h1 className="text-2xl font-bold mb-4">Plans</h1>
      <div className="space-y-4">
        {plans.map((plan) => (
          <div
            key={plan.id}
            className="p-4 bg-gray-800 rounded-lg cursor-pointer hover:bg-gray-700"
            onClick={() => selectPlan(plan)}
          >
            <h2 className="text-xl font-semibold">
              {plan.type} {plan.number}
            </h2>
            <p className="text-gray-400">NLT {plan.date}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default PlansList;
