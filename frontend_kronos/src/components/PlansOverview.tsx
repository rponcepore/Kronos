import React, { useState } from "react";
import PlansList from "./PlansList";
import { Plan } from "../types/planTypes";
import { useNavigate } from "react-router-dom";

interface PlansOverviewProps {
  plans: Plan[];
}

const PlansOverview: React.FC<PlansOverviewProps> = ({ plans }) => {
  const navigate = useNavigate();
  const [searchTerm, setSearchTerm] = useState("");

  const basePlans = plans.filter((plan) => plan.type === "PLAN");

  const filteredPlans = basePlans.filter((plan) =>
    plan.number.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const selectPlan = (plan: Plan) => {
    navigate(`/plans/${plan.id}`);
  };

  return (
    <div className="plans-overview">
      {/* Top Buttons */}
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

      {/* List of Plans */}
      <PlansList plans={filteredPlans} selectPlan={selectPlan} hideTypeBadge />
    </div>
  );
};

export default PlansOverview;



