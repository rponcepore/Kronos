import React, { useState, useEffect } from "react";
import { PlansList, PlansDetails } from "../components";
import { Plan } from "../types/backend_types/Plan";
import { PlanSummary } from "../types/frontend_types/PlanSummary";
import { KronosOrderSummary } from "../types/frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../types/frontend_types/ParagraphSummary";
import { kronosApiCall } from "../helper_methods/ApiCall";
import { KronosRequest } from "../types/networking_types/KronosRequest";
import "../styles/plans.css";

// Main container component for the Plans page
const PlansPage: React.FC = () => {
  // ----------------------------
  // Local state declarations
  // ----------------------------
  
  const [selectedPlan, setSelectedPlan] = useState<PlanSummary | null>(null);         // Tracks which plan is selected
  const [searchTerm, setSearchTerm] = useState("");                            // Tracks search input value
  const [sortBy, setSortBy] = useState<"default" | "alpha" | "year">("default"); // Sorting mode (alphabetical, year, or default)
  const [filterYear, setFilterYear] = useState<number | null>(null);           // Optional filter by fiscal year
  const [plans, setPlans] = useState<PlanSummary[]>([]);
  const [allOrders, setAllOrders] = useState<KronosOrderSummary[] | null>(null);
  const [allParagraphs, setAllParagraphs] = useState<ParagraphSummary[] | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // ----------------------------
  // Extract data from dummyData
  // ----------------------------

  useEffect(() => {
    async function fetchPlans() {
      try {
        setLoading(true);
        setError(null);

        // Fetch plans
        const plansReq: KronosRequest = {
          action: "get_plans",
          unit: "TEMPLT",
          plan_id: null,
          order_id: null,
          paragraph_id: null,
          task_id: null
        };
        const plansResponse = await kronosApiCall(plansReq);
        if (plansResponse.plans_vec) {
          setPlans(plansResponse.plans_vec);
        }

        // For now, set empty arrays for orders and paragraphs since they're not implemented
        setAllOrders([]);
        setAllParagraphs([]);
      } catch (err) {
        console.error('Error fetching data:', err);
        setError('Failed to fetch data from the server');
      } finally {
        setLoading(false);
      }
    }

    fetchPlans();
  }, []);

  // ----------------------------
  // Filter and sort functions
  // ----------------------------

  const filteredPlans = plans.filter(plan => {
    const matchesSearch = plan.data.name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesYear = !filterYear || plan.data.fiscal_year === filterYear;
    return matchesSearch && matchesYear;
  });

  const sortedPlans = [...filteredPlans].sort((a, b) => {
    switch (sortBy) {
      case "alpha":
        return a.data.name.localeCompare(b.data.name);
      case "year":
        return b.data.fiscal_year - a.data.fiscal_year;
      default:
        return 0;
    }
  });

  // ----------------------------
  // Render functions
  // ----------------------------

  if (loading) {
    return <div className="loading">Loading plans...</div>;
  }

  if (error) {
    return <div className="error">Error: {error}</div>;
  }

  return (
    <div className="plans-page-wrapper">
      <div className="top-controls">
        <input
          type="text"
          placeholder="Search plans..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="control-input"
        />
        <button 
          className="control-btn"
          onClick={() => setSortBy("alpha")}
        >
          Sort Alphabetically
        </button>
        <button 
          className="control-btn"
          onClick={() => setSortBy("year")}
        >
          Sort by Year
        </button>
        <select
          value={filterYear || ""}
          onChange={(e) => setFilterYear(e.target.value ? parseInt(e.target.value) : null)}
          className="control-input"
        >
          <option value="">All Years</option>
          {Array.from(new Set(plans.map(plan => plan.data.fiscal_year))).sort((a, b) => b - a).map(year => (
            <option key={year} value={year}>{year}</option>
          ))}
        </select>
      </div>
      <div className="plans-overview">
        <PlansList
          plans={sortedPlans}
          selectPlan={setSelectedPlan}
        />
        {selectedPlan && (
          <PlansDetails
            plan={selectedPlan}
            allOrders={allOrders}
            allParagraphs={allParagraphs}
            goBack={() => setSelectedPlan(null)}
          />
        )}
      </div>
    </div>
  );
};

export default PlansPage;



