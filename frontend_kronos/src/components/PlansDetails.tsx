import React, { useState } from "react";
import { Plan } from "../types/Plan";
import { Order } from "../types/Order";
import { Paragraph } from "../types/Paragraph";
import OrderDetails from "./OrderDetails"; // Import your new component

interface PlanDetailsProps {
  plan: Plan;
  allOrders: Order[];
  allParagraphs: Paragraph[];
  goBack: () => void;
}

const PlanDetails: React.FC<PlanDetailsProps> = ({
  plan,
  allOrders,
  allParagraphs,
  goBack,
}) => {
  const [selectedOrder, setSelectedOrder] = useState<Order | null>(null);

  const relatedOrders = allOrders.filter(
    (order) => order.parent_plan === plan.id
  );

  if (selectedOrder) {
    const relatedParagraphs = allParagraphs.filter(
      (p) => p.order === selectedOrder.id
    );

    return (
      <OrderDetails
        order={selectedOrder}
        plan={plan}
        paragraphs={relatedParagraphs}
        goBack={() => setSelectedOrder(null)}
      />
    );
  }

  return (
    <div className="plan-details">
      <div className="back-button-wrapper">
        <button className="back-button" onClick={goBack}>
          ← Back
        </button>
      </div>

      <h1>{plan.name}</h1>
      <p>
        <strong>Unit:</strong> {plan.unit}
      </p>
      <p>
        <strong>Fiscal Year:</strong> {plan.fiscal_year}
      </p>
      <p>
        <strong>Serial Number:</strong> {plan.serial_number}
      </p>
      <p>
        <strong>Classification:</strong> {plan.classification}
      </p>

      {relatedOrders.length > 0 && (
        <div className="related-orders">
          <h2>Orders</h2>
          <div className="related-orders-container">
            {relatedOrders.map((order) => (
              <div
                key={order.id}
                className={`plan-card ${order.order_type.toLowerCase()}`}
                onClick={() => setSelectedOrder(order)}
                style={{ cursor: "pointer" }}
              >
                <span className={`plan-type ${order.order_type.toLowerCase()}`}>
                  {order.order_type}
                </span>
                <h3>{`${plan.serial_number}-${order.serial_number}`}</h3>
                <p>
                  <strong>Published:</strong> {order.is_published ? "Yes" : "No"}
                </p>
                {order.derived_from && (
                  <p>
                    <strong>Derived From:</strong> {order.derived_from}
                  </p>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default PlanDetails;





