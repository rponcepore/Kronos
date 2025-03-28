import React, { useState } from "react";
import PlansList from "../components/PlansList";           // Component to render the list of plans
import PlanDetails from "../components/PlansDetails";       // Component to show plan details when selected
import { Plan } from "../types/Plan";                       // Plan type definition
import dummyData from "../dummy_data/dummy_orders";         // Static dummy data source (plans, orders, paragraphs)

// Main container component for the Plans page
const PlansPage: React.FC = () => {
  // ----------------------------
  // Local state declarations
  // ----------------------------
  
  const [selectedPlan, setSelectedPlan] = useState<Plan | null>(null);         // Tracks which plan is selected
  const [searchTerm, setSearchTerm] = useState("");                            // Tracks search input value
  const [sortBy, setSortBy] = useState<"default" | "alpha" | "year">("default"); // Sorting mode (alphabetical, year, or default)
  const [filterYear, setFilterYear] = useState<number | null>(null);           // Optional filter by fiscal year

  // ----------------------------
  // Extract data from dummyData
  // ----------------------------

  const plans = dummyData?.plans_vec ?? [];         // All plans
  const orders = dummyData?.orders_vec ?? [];       // All orders
  const paragraphs = dummyData?.paragraphs_vec ?? [];// All paragraph content

  // ----------------------------
  // Filter and sort logic
  // ----------------------------

  const filteredPlans = plans
    .filter((p) => !filterYear || p.fiscal_year === filterYear)                     // Filter by fiscal year if applied
    .filter((p) => p.name.toLowerCase().includes(searchTerm.toLowerCase()))        // Match search input to plan name
    .sort((a, b) => {
      if (sortBy === "alpha") return a.name.localeCompare(b.name);                 // Sort alphabetically
      if (sortBy === "year") return a.fiscal_year - b.fiscal_year;                // Sort by year
      return 0;                                                                    // Default (no sort)
    });

  return (
    <div className="plans-page-wrapper">
      {selectedPlan ? (
        // If a plan is selected, render PlanDetails with related orders and paragraphs
        <PlanDetails
          plan={selectedPlan}
          allOrders={orders}
          allParagraphs={paragraphs}
          goBack={() => setSelectedPlan(null)} // Return to plans list view
        />
      ) : (
        <>
          {/* Top controls for filter, sort, and search */}
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

            {/* Search input for plan name */}
            <input
              type="text"
              className="control-input"
              placeholder="Search plans..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>

          {/* Render list of plans after filtering/sorting */}
          <PlansList plans={filteredPlans} selectPlan={setSelectedPlan} />
        </>
      )}
    </div>
  );
};

export default PlansPage;



