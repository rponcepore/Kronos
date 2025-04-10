import React, { useState, useEffect } from "react";
import PlansList from "../components/Plans/PlansList";           // Component to render the list of plans
import PlanDetails from "../components/Plans/PlansDetails";       // Component to show plan details when selected
import { Plan } from "../types/backend_types/Plan";         // Plan type definition
import { PlanSummary } from "../types/frontend_types/PlanSummary";
import { kronosApiCall } from "../helper_methods/ApiCall";
import { KronosRequest } from "../types/networking_types/KronosRequest";
import OrderCard from "../components/OrderCard";
import { KronosApiMethod } from "../types/networking_types/KronosApiMethodEnums";
import NewPlanModal from "../components/Plans/NewPlanModal";

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
  const [showNewPlanModal, setShowNewPlanModal] = useState(false);


  // ----------------------------
  // Extract data from dummyData
  // ----------------------------

  useEffect(() => {
    async function fetchPlans() {
      try {
        setLoading(true);
        setError(null);
        const req: KronosRequest = {
          api_method: KronosApiMethod.get_plans,
          uic: "WJH8AA",
          plan_request: null,
          order_request: null,
          paragraph_request: null,
          task_request: null
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
        api_method: KronosApiMethod.get_order,
        uic: "WJH8AA",
        plan_request: null,
        order_request:{ order_id: orderId, parent_plan_id: null, order_type: null },
        paragraph_request: null,
        task_request: null
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
          });
        }
        setSelectedOrder(orderId);
      }
    } catch (err) {
      console.error('Error fetching order details:', err);
    }
  };

  //filtering and sorting logic for orders that are under a plan
  const filteredOrders = (selectedPlan?.orders || [])
  .filter((o) =>
    o.data.order_type.toLowerCase().includes(searchTerm.toLowerCase()) ||
    String(o.data.serial_number).includes(searchTerm)
  )
  .sort((a, b) => {
    if (sortBy === "alpha") return a.data.order_type.localeCompare(b.data.order_type);
    if (sortBy === "year") return a.data.serial_number - b.data.serial_number;
    return 0;
  });

  if (loading) {
    return <div className="p-6">Loading plans...</div>;
  }

  if (error) {
    return <div className="p-6 text-red-500">Error: {error}</div>;
  }

  //Buttons for the page where orders are displayed under a plan
  //mirros the same format of the plans page top buttons
  return (
    <div className="plans-page-wrapper">
      {selectedPlan && selectedPlan.data ? (
        <div className="plan-details">
          {/* Header section with plan info and buttons*/}
          <div className="top-controls mb-6 flex gap-4 flex-wrap">
          <button className="control-btn" onClick={() => setSelectedPlan(null)}>← Back to Plans</button>
          <button className="control-btn" onClick={() => setSortBy("alpha")}>
              Order A–Z
            </button>
            <button className="control-btn" onClick={() => setSortBy("year")}>
              Order by Year
            </button>
            <button className="control-btn" onClick={() => alert("Create WARNORD")}>
              <span className="plus-icon">＋</span> New WARNORD
            </button>
            <button className="control-btn" onClick={() => alert("Create OPORD")}>
              <span className="plus-icon">＋</span> New OPORD
            </button>
            <button className="control-btn" onClick={() => alert("Create FRAGORD")}>
              <span className="plus-icon">＋</span> New FRAGORD
            </button>

            {/*search field*/} 
            <input
              type="text"
              className="control-input"
              placeholder="Search orders..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>

          {/* Plan info section */}
          <div className="plan-info mb-4">
            <h1>{selectedPlan.data.name}</h1>
            <p><strong>UIC:</strong> {selectedPlan.data.uic}</p>
            <p><strong>Classification:</strong> {selectedPlan.data.classification}</p>
            <p><strong>Fiscal Year:</strong> FY {selectedPlan.data.fiscal_year}</p>
          </div>

          {/* Orders section */}
          <div className="orders-section">
            <h2>Orders</h2>
            <div className="orders-grid">
            {(selectedPlan.orders || []).map((order) => (
                <OrderCard
                  key={order.data.id}
                  order={order}
                  parentPlanFiscalYear={selectedPlan.data.fiscal_year}
                  selectOrder={() => handleOrderSelect(order.data.id)}
                  paragraphs={order.paragraphs || []}
                />
              ))}
            </div>
            {(selectedPlan.orders?.length ?? 0) === 0 && (
              <p style={{ fontStyle: "italic", opacity: 0.7, paddingTop: "1rem" }}>
                This plan has no orders yet.
              </p>
            )}
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
            {/*button for creating a new plan. Links to new modal for plan creation*/}
              <button className="control-btn new-plan-btn" onClick={() => setShowNewPlanModal(true)}>
                <span className="plus-icon">＋</span> New Plan
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

          {showNewPlanModal && (
            <NewPlanModal
              onClose={() => setShowNewPlanModal(false)}
              onSuccess={() => window.location.reload()}
            />
         )}
        </>
      )}
    </div>
  );
};

export default PlansPage;



