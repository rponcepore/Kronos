import React from "react";
import { Order } from "../types/Order";
import { Paragraph } from "../types/Paragraph";
import { Plan } from "../types/Plan";

interface OrderDetailsProps {
  order: Order;
  plan: Plan;
  paragraphs: Paragraph[];
  goBack: () => void;
}

const OrderDetails: React.FC<OrderDetailsProps> = ({ order, plan, paragraphs, goBack }) => {
  const renderParagraphs = () =>
    paragraphs
      .sort((a, b) => a.ordinal_sequence - b.ordinal_sequence)
      .map((p) => (
        <div key={p.id} style={{ marginBottom: "1.5rem" }}>
          <h3>{p.title}</h3>
          <p>{p.text}</p>
        </div>
      ));

  return (
    <div className="plan-details">
      <div className="back-button-wrapper">
        <button className="back-button" onClick={goBack}>
          ← Back
        </button>
      </div>

      <h1>{`${order.order_type} ${plan.serial_number}-${order.serial_number}`}</h1>
      <p><strong>Parent Plan:</strong> {plan.name}</p>
      <p><strong>Published:</strong> {order.is_published ? "Yes" : "No"}</p>
      {order.derived_from && <p><strong>Derived From:</strong> Order ID {order.derived_from}</p>}

      <hr style={{ margin: "1.5rem 0" }} />
      {renderParagraphs()}
    </div>
  );
};

export default OrderDetails;
