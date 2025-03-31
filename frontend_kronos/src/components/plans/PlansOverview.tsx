import React, { useState } from "react";
import PlansList from "./PlansList";
import { Plan } from "../../types/backend_types/Plan";
import { PlanSummary } from "../../types/frontend_types/PlanSummary";
import { useNavigate } from "react-router-dom";

interface PlansOverviewProps {
  plans: Plan[];
}

const PlansOverview: React.FC<PlansOverviewProps> = ({ plans }) => {
  const navigate = useNavigate();
  const [searchTerm, setSearchTerm] = useState("");

  const filteredPlans = plans.filter((plan) =>
    plan.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const selectPlan = (plan: PlanSummary) => {
    navigate(`/plans/${plan.data.id}`);
  };

  // Transform Plan[] into PlanSummary[]
  const planSummaries: PlanSummary[] = filteredPlans.map(plan => ({
    data: plan,
    orders: []
  }));

  return (
    <div className="p-6">
      <div className="flex flex-wrap gap-4 mb-6">
        <button className="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200">
          Filter
        </button>
        <button className="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200">
          Order By
        </button>
        <input
          type="text"
          placeholder="Search"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="px-4 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        />
        <button className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
          New Plan ➕
        </button>
      </div>

      <PlansList plans={planSummaries} selectPlan={selectPlan} />
    </div>
  );
};

export default PlansOverview;




