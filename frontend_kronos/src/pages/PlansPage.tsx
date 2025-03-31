import React, { useState, useEffect } from "react";
import { PlansList, PlansDetails } from "../components";
import { Plan } from "../types/backend_types/Plan";
import { PlanSummary } from "../types/frontend_types/PlanSummary";
import { KronosOrderSummary } from "../types/frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../types/frontend_types/ParagraphSummary";
import { kronosApiCall } from "../helper_methods/ApiCall";
import { KronosRequest } from "../types/networking_types/KronosRequest";
import "../styles/plans.css";
import PlansOverview from "../components/plans/PlansOverview";

// Main container component for the Plans page
const PlansPage: React.FC = () => {
  // ----------------------------
  // Local state declarations
  // ----------------------------
  
  const [selectedPlan, setSelectedPlan] = useState<PlanSummary | null>(null);         // Tracks which plan is selected
  const [searchTerm, setSearchTerm] = useState("");                            // Tracks search input value
  const [sortBy, setSortBy] = useState<"default" | "alpha" | "year">("default"); // Sorting mode (alphabetical, year, or default)
  const [filterYear, setFilterYear] = useState<number | null>(null);           // Optional filter by fiscal year
  const [plans, setPlans] = useState<Plan[]>([]);
  const [allOrders, setAllOrders] = useState<KronosOrderSummary[] | null>(null);
  const [allParagraphs, setAllParagraphs] = useState<ParagraphSummary[] | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // ----------------------------
  // Extract data from dummyData
  // ----------------------------

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        setError(null);

        // Set up the request for plans
        const plansRequest: KronosRequest = {
          action: "get_plans",
          unit: "WJH8AA",
          plan_id: null,
          order_id: null,
          paragraph_id: null,
          task_id: null
        };

        // Make the API call
        const response = await kronosApiCall(plansRequest);

        // Check if the response is valid
        if (response && response.plans_vec) {
          // Extract just the Plan objects from the PlanSummary array
          const planObjects = response.plans_vec.map((summary: PlanSummary) => summary.data);
          setPlans(planObjects);
          
          // Collect all orders from all plans
          const allOrdersFromPlans = response.plans_vec.flatMap((summary: PlanSummary) => summary.orders);
          setAllOrders(allOrdersFromPlans);
        } else {
          setError("Invalid response format from server");
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : "An error occurred");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  // ----------------------------
  // Filter and sort functions
  // ----------------------------

  const filteredPlans = plans.filter(plan => {
    const matchesSearch = plan.name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesYear = !filterYear || plan.fiscal_year === filterYear;
    return matchesSearch && matchesYear;
  });

  const sortedPlans = [...filteredPlans].sort((a, b) => {
    switch (sortBy) {
      case "alpha":
        return a.name.localeCompare(b.name);
      case "year":
        return b.fiscal_year - a.fiscal_year;
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
          {Array.from(new Set(plans.map(plan => plan.fiscal_year))).sort((a, b) => b - a).map(year => (
            <option key={year} value={year}>{year}</option>
          ))}
        </select>
      </div>
      <div className="plans-overview">
        <PlansOverview 
          plans={sortedPlans} 
          allOrders={allOrders}
          allParagraphs={allParagraphs}
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



