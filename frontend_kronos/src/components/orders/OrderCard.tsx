import React from "react";
import { KronosOrderSummary } from "../../types/frontend_types/KronosOrderSummary";
import { getOrderSerialDisplay } from "../../helper_methods/format";

// ---------------------------------------
// Props interface for the OrderCard component
// ---------------------------------------
interface OrderCardProps {
  order: KronosOrderSummary;
  parentPlanFiscalYear: number;
  selectOrder: (order: KronosOrderSummary) => void;
}

// ---------------------------------------
// OrderCard Component
// Displays a single OPORD, WARNO, or FRAGO as a styled card
// ---------------------------------------
const OrderCard: React.FC<OrderCardProps> = ({
  order,
  parentPlanFiscalYear,
  selectOrder,
}) => {
  // Generate the full serial number (e.g., "25-01-02") using plan and order info
  const serial = getOrderSerialDisplay(
    parentPlanFiscalYear,
    order.data.parent_plan,
    order.data.serial_number
  );

  return (
    <div 
      className="bg-white rounded-lg shadow-sm p-6 cursor-pointer hover:shadow-md transition-shadow duration-200 flex flex-col items-center text-center"
      onClick={() => selectOrder(order)}
    >
      <div className="text-xl font-medium mb-4">
        {order.data.order_type}
      </div>

      <div className="text-3xl font-bold mb-4">
        {serial}
      </div>

      <div className="text-gray-600">
        Published: {order.data.is_published ? "Yes" : "No"}
      </div>

      {order.data.derived_from && (
        <div className="text-gray-600 mt-2">
          Derived From: {order.data.derived_from}
        </div>
      )}
    </div>
  );
};

export default OrderCard;


