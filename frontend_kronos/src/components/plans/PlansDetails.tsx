import React, { useState } from "react";
import { PlanSummary } from "../../types/frontend_types/PlanSummary";
import { KronosOrderSummary } from "../../types/frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../../types/frontend_types/ParagraphSummary";
import { getPlanSerialDisplay } from "../../helper_methods/format";

// Subcomponents for rendering orders and order details
import OrderDetails from "../orders/OrderDetails";
import OrderCard from "../orders/OrderCard";

// Props that PlanDetails expects to receive
interface PlanDetailsProps {
  plan: PlanSummary;
  allOrders: KronosOrderSummary[] | null;
  allParagraphs: ParagraphSummary[] | null;
  goBack: () => void;
}

// Main component to display detailed information about a selected plan
const PlanDetails: React.FC<PlanDetailsProps> = ({
  plan,
  allOrders = [], // Default to empty array if null
  allParagraphs = [], // Default to empty array if null
  goBack,
}) => {
  // Local state to track which order is selected
  const [selectedOrder, setSelectedOrder] = useState<KronosOrderSummary | null>(null);
  const [viewingRelatedOrders, setViewingRelatedOrders] = useState(false);
  const [relatedOrders, setRelatedOrders] = useState<KronosOrderSummary[]>([]);

  // Filter orders to only show those associated with this plan
  const planOrders = (allOrders || []).filter(
    (order) => order?.data?.parent_plan === plan.data.id
  );

  // Filter paragraphs to only show those associated with this plan's orders
  const planParagraphs = (allParagraphs || []).filter((paragraph) =>
    planOrders.some((order) => order?.data?.id === paragraph?.data?.order)
  );

  // Function to find related orders
  const findRelatedOrders = (order: KronosOrderSummary) => {
    if (!order?.data) return;
    
    const related = planOrders.filter(o => {
      if (!o?.data) return false;
      // If the order is derived from another order, show that order
      if (order.data.derived_from === o.data.id) {
        return true;
      }
      // If another order is derived from this order, show that order
      if (o.data.derived_from === order.data.id) {
        return true;
      }
      return false;
    });
    setRelatedOrders(related);
    setViewingRelatedOrders(true);
  };

  // Function to handle order selection
  const handleOrderSelect = (order: KronosOrderSummary) => {
    setSelectedOrder(order);
    findRelatedOrders(order);
  };

  // Function to go back to all orders
  const handleBackToOrders = () => {
    setSelectedOrder(null);
    setViewingRelatedOrders(false);
    setRelatedOrders([]);
  };

  return (
    <div className="plan-details p-6">
      {/* Header section with plan info and back button */}
      <div className="plan-header mb-8">
        <h1 className="text-3xl font-bold mb-4">{plan.data.name}</h1>
        <div className="text-gray-600 mb-4">
          <p>Unit: {plan.data.unit}</p>
          <p>Fiscal Year: {plan.data.fiscal_year}</p>
        </div>
        <button 
          onClick={goBack}
          className="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200"
        >
          Back to Plans
        </button>
      </div>

      {/* Orders section */}
      <div className="orders-section">
        <h2 className="text-2xl font-semibold mb-6">Orders</h2>
        {viewingRelatedOrders && (
          <button 
            onClick={handleBackToOrders}
            className="mb-6 px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200"
          >
            ← Back to All Orders
          </button>
        )}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {(viewingRelatedOrders ? relatedOrders : planOrders).map((order) => (
            order?.data && (
              <OrderCard
                key={order.data.id}
                order={order}
                parentPlanFiscalYear={plan.data.fiscal_year}
                selectOrder={handleOrderSelect}
              />
            )
          ))}
        </div>
      </div>

      {/* Order details modal */}
      {selectedOrder?.data && (
        <OrderDetails
          order={selectedOrder}
          allParagraphs={planParagraphs}
          goBack={handleBackToOrders}
        />
      )}
    </div>
  );
};

export default PlanDetails;

