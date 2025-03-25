import React, { useState } from "react";
import PlansList from "../components/PlansList";
import PlanDetails from "../components/PlansDetails";
import { Plan } from "../types/Plan";
import dummyData from "../dummy_data/dummy_orders"; // ✅ Correct import

const PlansPage: React.FC = () => {
  const [selectedPlan, setSelectedPlan] = useState<Plan | null>(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [sortBy, setSortBy] = useState<"default" | "alpha" | "year">("default");
  const [filterYear, setFilterYear] = useState<number | null>(null);

  // ✅ Fallback with default values, ensuring TS knows these arrays always exist
  const plans = dummyData?.plans_vec ?? [];
  const orders = dummyData?.orders_vec ?? [];
  const paragraphs = dummyData?.paragraphs_vec ?? [];

  // ✅ Filter and sort logic
  const filteredPlans = plans
    .filter((p) => !filterYear || p.fiscal_year === filterYear)
    .filter((p) => p.name.toLowerCase().includes(searchTerm.toLowerCase()))
    .sort((a, b) => {
      if (sortBy === "alpha") return a.name.localeCompare(b.name);
      if (sortBy === "year") return a.fiscal_year - b.fiscal_year;
      return 0;
    });

  return (
    <div className="p-6 mt-16">
      {selectedPlan ? (
        <PlanDetails
          plan={selectedPlan}
          allOrders={orders}
          allParagraphs={paragraphs}
          goBack={() => setSelectedPlan(null)}
        />
      ) : (
        <>
          <div className="top-controls mb-6 flex gap-4 flex-wrap">
            <button className="control-btn" onClick={() => setFilterYear(null)}>
              All Years
            </button>
            <button className="control-btn" onClick={() => setFilterYear(2025)}>
              FY25
            </button>
            <button className="control-btn" onClick={() => setSortBy("alpha")}>
              Order A–Z
            </button>
            <button className="control-btn" onClick={() => setSortBy("year")}>
              Order by Year
            </button>
            <input
              type="text"
              className="control-input"
              placeholder="Search plans..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>

          <PlansList plans={filteredPlans} selectPlan={setSelectedPlan} />
        </>
      )}
    </div>
  );
};

export default PlansPage;


