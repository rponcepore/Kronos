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

  // Filter orders to only show those associated with this plan
  const planOrders = (allOrders || []).filter(
    (order) => order.data.parent_plan === plan.data.id
  );

  // Filter paragraphs to only show those associated with this plan's orders
  const planParagraphs = (allParagraphs || []).filter((paragraph) =>
    planOrders.some((order) => order.data.id === paragraph.data.order)
  );

  return (
    <div className="plan-details">
      {/* Header section with plan info and back button */}
      <div className="plan-header">
        <h1>{plan.data.name}</h1>
        <p>Unit: {plan.data.unit}</p>
        <p>Fiscal Year: {plan.data.fiscal_year}</p>
        <button onClick={goBack}>Back to Plans</button>
      </div>

      {/* Orders section */}
      <div className="orders-section">
        <h2>Orders</h2>
        <div className="orders-grid">
          {planOrders.map((order) => (
            <OrderCard
              key={order.data.id}
              order={order}
              parentPlanFiscalYear={plan.data.fiscal_year}
              selectOrder={setSelectedOrder}
            />
          ))}
        </div>
      </div>

      {/* Order details modal */}
      {selectedOrder && (
        <OrderDetails
          order={selectedOrder}
          allParagraphs={planParagraphs}
          goBack={() => setSelectedOrder(null)}
        />
      )}
    </div>
  );
};

export default PlanDetails;

