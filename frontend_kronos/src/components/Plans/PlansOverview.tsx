import React, { useState } from "react";
import PlansList from "./PlansList";
import { PlanSummary } from "../../types/frontend_types/PlanSummary";
import { Plan } from "../../types/backend_types/Plan";
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

  // Transform Plan[] to PlanSummary[]
  const planSummaries: PlanSummary[] = filteredPlans.map(plan => ({
    data: plan,
    orders: []
  }));

  return (
    <div className="plans-overview">
      <div className="top-controls" style={{ display: "flex", gap: "10px", marginBottom: "20px" }}>
        <button className="control-btn">Filter</button>
        <button className="control-btn">Order By</button>
        <input
          type="text"
          placeholder="Search"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="control-input"
        />
        <button className="control-btn">New Plan ➕</button>
      </div>

      <PlansList plans={planSummaries} selectPlan={selectPlan} />
    </div>
  );
};

export default PlansOverview;




