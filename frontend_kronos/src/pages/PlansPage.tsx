import React, { useState } from "react";
import PlansList from "../components/PlansList";
import PlanDetails from "../components/PlansDetails";
import { Plan } from "../types/planTypes";
import plansData from "../data/plansData";

const PlansPage: React.FC = () => {
  const [selectedPlan, setSelectedPlan] = useState<Plan | null>(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [sortBy, setSortBy] = useState<"default" | "alpha" | "published">("default");
  const [filterYear, setFilterYear] = useState<string | null>(null);

  // Get base plans only (type === PLAN)
  const basePlans = plansData.filter((p) => p.type === "PLAN");

  // Apply filters and sorting
  const filteredPlans = basePlans
    .filter((p) =>
      !filterYear || p.number.startsWith(filterYear)
    )
    .filter((p) =>
      p.number.toLowerCase().includes(searchTerm.toLowerCase()) ||
      p.mission.toLowerCase().includes(searchTerm.toLowerCase())
    )
    .sort((a, b) => {
      if (sortBy === "alpha") return a.number.localeCompare(b.number);
      if (sortBy === "published") return a.published.localeCompare(b.published);
      return 0;
    });

  return (
    <div className="p-6 mt-16">
      {selectedPlan ? (
        <PlanDetails
          plan={selectedPlan}
          allPlans={plansData}
          goBack={() => setSelectedPlan(null)}
        />
      ) : (
        <>
          {/* Button Bar */}
          <div className="top-controls mb-6 flex gap-4 flex-wrap">
            <button className="control-btn" onClick={() => setFilterYear(null)}>All Years</button>
            <button className="control-btn" onClick={() => setFilterYear("25")}>FY25</button>
            <button className="control-btn" onClick={() => setSortBy("alpha")}>Order A-Z</button>
            <button className="control-btn" onClick={() => setSortBy("published")}>Order by Published</button>
            <input
              type="text"
              className="control-input"
              placeholder="Search plans..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
            <button className="control-btn">New Plan ➕</button>
          </div>

          {/* Plans List */}
          <PlansList plans={filteredPlans} selectPlan={setSelectedPlan} hideTypeBadge />
        </>
      )}
    </div>
  );
};

export default PlansPage;
