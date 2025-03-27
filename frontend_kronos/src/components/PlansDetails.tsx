import React, { useState } from "react";

// Types for props
import { Plan } from "../types/Plan";
import { KronosOrder } from "../types/KronosOrder";
import { Paragraph } from "../types/Paragraph";

// Subcomponents for rendering orders and order details
import OrderDetails from "./OrderDetails";
import OrderCard from "./OrderCard";

// Utility functions to format serial numbers
import { getPlanSerialDisplay, getOrderSerialDisplay } from "../helper_methods/format";


// Props that PlanDetails expects to receive
interface PlanDetailsProps {
  plan: Plan;                            // The selected base plan
  allOrders: KronosOrder[];             // All orders from the database/dummy data
  allParagraphs: Paragraph[];           // All paragraphs for every order
  goBack: () => void;                   // Callback to return to the previous view
}

const PlanDetails: React.FC<PlanDetailsProps> = ({
  plan,
  allOrders,
  allParagraphs,
  goBack,
}) => {

  // State to track if an individual order (OPORD, FRAGO, WARNO) has been selected
  const [selectedOrder, setSelectedOrder] = useState<KronosOrder | null>(null);

  // Filter allOrders to just the ones related to this specific base plan
  const relatedOrders = allOrders.filter(
    (order) => order.parent_plan === plan.id
  );

  return (
    <div className="plan-details">
      {/* Top-left back button to exit plan detail view */}
      <div className="back-button-wrapper">
        <button className="back-button" onClick={goBack}>
          ← Back
        </button>
      </div>

      {/* If an order has been selected, show the OrderDetails view */}
      {selectedOrder ? (
        <OrderDetails
          order={selectedOrder}
          parentPlan={plan}
          // Filter paragraphs to just the ones belonging to the selected order
          paragraphs={allParagraphs.filter(
            (p) => p.order === selectedOrder.id
          )}
          goBack={() => setSelectedOrder(null)}  // Clicking "Back" in OrderDetails resets selectedOrder
        />
      ) : (
        <>
          {/* Base Plan Metadata */}
          <h1>{plan.name}</h1>
          <p>
            <strong>Unit:</strong> {plan.unit}
          </p>
          <p>
            <strong>Fiscal Year:</strong> {plan.fiscal_year}
          </p>
          <p>
            <strong>Serial Number:</strong>{" "}
            {getPlanSerialDisplay(plan.fiscal_year, plan.serial_number)}
          </p>
          <p>
            <strong>Classification:</strong> {plan.classification}
          </p>

          {/* Related orders (OPORDs, WARNOs, FRAGOs) for this plan */}
          {relatedOrders.length > 0 && (
            <div className="related-orders">
              <h2>Orders</h2>

              {/* Container for all OrderCard components */}
              <div className="related-orders-container">
                {relatedOrders.map((order) => {
                  
                  // If this order is a FRAGO (i.e., derived from another order), build its derived serial number
                  const derivedSerial =
                    order.derived_from !== null
                      ? getOrderSerialDisplay(
                          plan.fiscal_year,
                          plan.serial_number,
                          allOrders.find((o) => o.id === order.derived_from)
                            ?.serial_number ?? null
                        )
                      : undefined;

                  return (
                    <OrderCard
                      key={order.id}
                      order={order}
                      fiscalYear={plan.fiscal_year}
                      planSerial={plan.serial_number}
                      derivedSerial={derivedSerial}
                      onClick={() => setSelectedOrder(order)} // Clicking an OrderCard sets the selected order
                    />
                  );
                })}
              </div>
            </div>
          )}
        </>
      )}
    </div>
  );
};

export default PlanDetails;

