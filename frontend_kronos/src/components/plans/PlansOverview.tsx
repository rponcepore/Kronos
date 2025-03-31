import React, { useState } from "react";
import PlansList from "./PlansList";
import PlansDetails from "./PlansDetails";
import { Plan } from "../../types/backend_types/Plan";
import { PlanSummary } from "../../types/frontend_types/PlanSummary";
import { KronosOrderSummary } from "../../types/frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../../types/frontend_types/ParagraphSummary";

interface PlansOverviewProps {
  plans: Plan[];
  allOrders: KronosOrderSummary[] | null;
  allParagraphs: ParagraphSummary[] | null;
}

const PlansOverview: React.FC<PlansOverviewProps> = ({ 
  plans, 
  allOrders = [], 
  allParagraphs = [] 
}) => {
  const [searchTerm, setSearchTerm] = useState("");
  const [selectedPlan, setSelectedPlan] = useState<PlanSummary | null>(null);

  const filteredPlans = plans.filter((plan) =>
    plan.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  // Transform Plan[] into PlanSummary[]
  const planSummaries: PlanSummary[] = filteredPlans.map(plan => {
    // Find orders that belong to this plan
    const planOrders = (allOrders || []).filter(order => {
      if (!order?.data) return false;
      return order.data.parent_plan === plan.id;
    });

    return {
      data: plan,
      orders: planOrders
    };
  });

  const selectPlan = (plan: PlanSummary) => {
    setSelectedPlan(plan);
  };

  const goBack = () => {
    setSelectedPlan(null);
  };

  if (selectedPlan) {
    return (
      <PlansDetails
        plan={selectedPlan}
        allOrders={allOrders}
        allParagraphs={allParagraphs}
        goBack={goBack}
      />
    );
  }

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




