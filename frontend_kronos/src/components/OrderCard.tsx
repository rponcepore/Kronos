import React from "react";
import { KronosOrder } from "../types/KronosOrder";                  // Type definition for order object
import { getOrderSerialDisplay } from "../helper_methods/format";   // Utility function to format serial numbers
import "../styles/orderCard.css";                                   // Styling for this component

// ---------------------------------------
// Props interface for the OrderCard component
// ---------------------------------------
interface OrderCardProps {
  order: KronosOrder;             // The order object to display
  fiscalYear: number;             // Fiscal year of the parent plan
  planSerial: number;             // Serial number of the parent plan
  derivedSerial?: string;         // Optional: serial number of the order this one is derived from
  onClick: () => void;            // Function to execute when the card is clicked
}

// ---------------------------------------
// OrderCard Component
// Displays a single OPORD, WARNO, or FRAGO as a styled card
// ---------------------------------------
const OrderCard: React.FC<OrderCardProps> = ({
  order,
  fiscalYear,
  planSerial,
  derivedSerial,
  onClick,
}) => {
  // Generate the full serial number (e.g., "25-01-02") using plan and order info
  const serial = getOrderSerialDisplay(fiscalYear, planSerial, order.serial_number);

  return (
    <div className="order-card" onClick={onClick}>
      {/* Header section with badge and serial */}
      <div className="order-card-header">
        <span className={`plan-type ${order.order_type.toLowerCase()}`}>
          {order.order_type}  {/* OPORD, WARNO, or FRAGO */}
        </span>
        <h3>{serial}</h3>
      </div>

      {/* Publication status */}
      <p><strong>Published:</strong> {order.is_published ? "Yes" : "No"}</p>

      {/* Derived From (only for FRAGOs) */}
      {derivedSerial && (
        <p><strong>Derived From:</strong> {derivedSerial}</p>
      )}
    </div>
  );
};

export default OrderCard;


