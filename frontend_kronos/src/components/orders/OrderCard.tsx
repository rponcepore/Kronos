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
      className="bg-white rounded-lg shadow-md p-4 cursor-pointer hover:shadow-lg transition-shadow duration-200"
      onClick={() => selectOrder(order)}
    >
      <div className="flex items-center justify-between mb-2">
        <span className={`px-2 py-1 rounded text-sm font-semibold ${
          order.data.order_type.toLowerCase() === 'opord' ? 'bg-blue-100 text-blue-800' :
          order.data.order_type.toLowerCase() === 'warnord' ? 'bg-yellow-100 text-yellow-800' :
          'bg-green-100 text-green-800'
        }`}>
          {order.data.order_type}
        </span>
        <h3 className="text-lg font-bold">{serial}</h3>
      </div>

      <p className="text-sm text-gray-600">
        <span className="font-medium">Published:</span> {order.data.is_published ? "Yes" : "No"}
      </p>

      {order.data.derived_from && (
        <p className="text-sm text-gray-600 mt-1">
          <span className="font-medium">Derived From:</span> {order.data.derived_from}
        </p>
      )}
    </div>
  );
};

export default OrderCard;


