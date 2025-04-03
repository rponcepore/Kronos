import React, { useState, useEffect } from "react";
import PlansList from "../components/Plans/PlansList";           // Component to render the list of plans
import PlanDetails from "../components/Plans/PlansDetails";       // Component to show plan details when selected
import { Plan } from "../types/backend_types/Plan";         // Plan type definition
import { PlanSummary } from "../types/frontend_types/PlanSummary";
import { kronosApiCall } from "../helper_methods/ApiCall";
import { KronosRequest } from "../types/networking_types/KronosRequest";
import OrderCard from "../components/OrderCard";

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
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedOrder, setSelectedOrder] = useState<number | null>(null);

  // ----------------------------
  // Extract data from dummyData
  // ----------------------------

  useEffect(() => {
    async function fetchPlans() {
      try {
        setLoading(true);
        setError(null);
        const req: KronosRequest = {
          action: "get_plans",
          unit: "WJH8AA",
          plan_id: null,
          order_id: null,
          paragraph_id: null,
          task_id: null
        };
        const response = await kronosApiCall(req);
        console.log('Response:', response);
        
        if (response.plans_vec) {
          console.log('Plans received:', response.plans_vec);
          setPlans(response.plans_vec);
        } else {
          console.log('No plans received from API');
          setError('No plans were returned from the server');
        }
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Failed to fetch plans';
        console.error('Error fetching plans:', err);
        setError(errorMessage);
      } finally {
        setLoading(false);
      }
    }

    fetchPlans();
  }, []);

  // ----------------------------
  // Filter and sort logic
  // ----------------------------

  const filteredPlans = plans
    .filter((p) => !filterYear || p.data.fiscal_year === filterYear)                     // Filter by fiscal year if applied
    .filter((p) => p.data.name.toLowerCase().includes(searchTerm.toLowerCase()))        // Match search input to plan name
    .sort((a, b) => {
      if (sortBy === "alpha") return a.data.name.localeCompare(b.data.name);                 // Sort alphabetically
      if (sortBy === "year") return a.data.fiscal_year - b.data.fiscal_year;                // Sort by year
      return 0;                                                                    // Default (no sort)
    });

  const handleOrderSelect = async (orderId: number) => {
    try {
      const req: KronosRequest = {
        action: "get_order",
        unit: "WJH8AA",
        plan_id: null,
        order_id: orderId,
        paragraph_id: null,
        task_id: null
      };
      const response = await kronosApiCall(req);
      if (response.orders_vec && response.orders_vec[0]) {
        const order = response.orders_vec[0];
        // Update the selected plan's orders with the new paragraphs
        if (selectedPlan) {
          const updatedOrders = selectedPlan.orders.map(o => 
            o.data.id === orderId ? order : o
          );
          setSelectedPlan({
            ...selectedPlan,
            orders: updatedOrders,
            paragraphs: order.paragraphs || []
          });
        }
        setSelectedOrder(orderId);
      }
    } catch (err) {
      console.error('Error fetching order details:', err);
    }
  };

  if (loading) {
    return <div className="p-6">Loading plans...</div>;
  }

  if (error) {
    return <div className="p-6 text-red-500">Error: {error}</div>;
  }

  return (
    <div className="plans-page-wrapper">
      {selectedPlan ? (
        <div className="plan-details">
          {/* Header section with plan info and back button */}
          <div className="plan-header">
            <h1>{selectedPlan.data.name}</h1>
            <p>Unit: {selectedPlan.data.unit}</p>
            <p>Fiscal Year: {selectedPlan.data.fiscal_year}</p>
            <button onClick={() => setSelectedPlan(null)}>Back to Plans</button>
          </div>

          {/* Orders section */}
          <div className="orders-section">
            <h2>Orders</h2>
            <div className="orders-grid">
              {selectedPlan.orders.map((order) => (
                <OrderCard
                  key={order.data.id}
                  order={order}
                  parentPlanFiscalYear={selectedPlan.data.fiscal_year}
                  selectOrder={() => handleOrderSelect(order.data.id)}
                  paragraphs={selectedPlan.paragraphs?.filter((p) => p.data.order === order.data.id) || []}
                />
              ))}
            </div>
          </div>
        </div>
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



