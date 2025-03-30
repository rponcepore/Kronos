import React from "react";
import { KronosOrderSummary } from "../types/frontend_types/KronosOrderSummary";
import { getOrderSerialDisplay } from "../helper_methods/format";
import "../styles/orderCard.css";

// ---------------------------------------
// Props interface for the OrderCard component
// ---------------------------------------
interface OrderCardProps {
  order: KronosOrderSummary;
  selectOrder: (order: KronosOrderSummary) => void;
}

// ---------------------------------------
// OrderCard Component
// Displays a single OPORD, WARNO, or FRAGO as a styled card
// ---------------------------------------
const OrderCard: React.FC<OrderCardProps> = ({
  order,
  selectOrder,
}) => {
  // Generate the full serial number (e.g., "25-01-02") using plan and order info
  const serial = getOrderSerialDisplay(
    order.data.fiscal_year,
    order.data.serial_number
  );

  return (
    <div className="order-card" onClick={() => selectOrder(order)}>
      {/* Header section with badge and serial */}
      <div className="order-card-header">
        <span className={`plan-type ${order.data.order_type.toLowerCase()}`}>
          {order.data.order_type}  {/* OPORD, WARNO, or FRAGO */}
        </span>
        <h3>{serial}</h3>
      </div>

      {/* Publication status */}
      <p><strong>Published:</strong> {order.data.is_published ? "Yes" : "No"}</p>

      {/* Derived From (only for FRAGOs) */}
      {order.data.derived_from && (
        <p><strong>Derived From:</strong> {order.data.derived_from}</p>
      )}
    </div>
  );
};

export default OrderCard;


