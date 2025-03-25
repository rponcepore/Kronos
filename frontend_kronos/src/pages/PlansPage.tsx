import React, { useState } from "react";
import PlansList from "../components/PlansList";
import PlanDetails from "../components/PlansDetails";
import { Plan } from "../types/Plan";
import { plans, orders } from "../dummy_data/dummy_orders";

const PlansPage: React.FC = () => {
  const [selectedPlan, setSelectedPlan] = useState<Plan | null>(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [sortBy, setSortBy] = useState<"default" | "alpha" | "published">("default");
  const [filterYear, setFilterYear] = useState<number | null>(null);

  // Get base plans only (type === PLAN)
  const basePlans = plans; //.filter((p) => p.type === "PLAN");

  // Apply filters and sorting
  const filteredPlans = basePlans
    .filter((p: Plan) =>
      !filterYear || p.fiscal_year == (filterYear)
    )
    /*
    .filter((p: Plan) =>
      p.serial_number == searchTerm ||
      p.mission.toLowerCase().includes(searchTerm.toLowerCase())
    )
      
    .sort((a: Plan, b: Plan) => {
      if (sortBy === "alpha") return a.serial_number.compare(b.serial_number);
      if (sortBy === "published") return a.published.localeCompare(b.published);
      return 0;
    });
    */

  return (
    <div className="p-6 mt-16">
      {selectedPlan ? (
        <PlanDetails
          plan={selectedPlan}
          orders={orders}
          goBack={() => setSelectedPlan(null)}
        />
      ) : (
        <>
          {/* Button Bar */}
          <div className="top-controls mb-6 flex gap-4 flex-wrap">
            <button className="control-btn" onClick={() => setFilterYear(null)}>All Years</button>
            <button className="control-btn" onClick={() => setFilterYear(25)}>FY25</button>
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
