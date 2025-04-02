import React, { useState } from "react";
import { KronosOrderSummary } from "../types/frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../types/frontend_types/ParagraphSummary";
import { getOrderSerialDisplay } from "../helper_methods/format";
import "../styles/orderCard.css";

// ---------------------------------------
// Props interface for the OrderCard component
// ---------------------------------------
interface OrderCardProps {
  order: KronosOrderSummary;
  parentPlanFiscalYear: number;
  selectOrder: (order: KronosOrderSummary) => void;
  paragraphs: ParagraphSummary[] | null;
}

// ---------------------------------------
// OrderCard Component
// Displays a single OPORD, WARNO, or FRAGO as a styled card
// ---------------------------------------
const OrderCard: React.FC<OrderCardProps> = ({
  order,
  parentPlanFiscalYear,
  selectOrder,
  paragraphs,
}) => {
  const [showModal, setShowModal] = useState(false);

  // Generate the full serial number (e.g., "25-01-02") using plan and order info
  const serial = getOrderSerialDisplay(
    parentPlanFiscalYear,
    order.data.parent_plan,
    order.data.serial_number
  );

  // Find the mission paragraph
  const missionParagraph = paragraphs?.find(p => p.data.title.toLowerCase() === "mission");

  const handleClick = (e: React.MouseEvent) => {
    e.preventDefault();
    setShowModal(true);
    selectOrder(order);
  };

  return (
    <>
      <div className="order-card" onClick={handleClick}>
        <div className="order-card-content">
          {/* First column: Order type and serial */}
          <div className="order-card-column">
            <div className="order-card-header">
              <span className={`plan-type ${order.data.order_type.toLowerCase()}`}>
                {order.data.order_type}
              </span>
              <h3>{serial}</h3>
            </div>
          </div>

          {/* Second column: Publication status */}
          <div className="order-card-column">
            <p><strong>Published:</strong> {order.data.is_published ? "Yes" : "No"}</p>
          </div>

          {/* Third column: Mission paragraph */}
          <div className="order-card-column mission-column">
            {missionParagraph && (
              <div className="mission-text">
                <strong>Mission:</strong> {missionParagraph.data.text}
              </div>
            )}
          </div>
        </div>
      </div>

      {showModal && (
        <div className="modal-overlay" onClick={() => setShowModal(false)}>
          <div className="modal-content" onClick={e => e.stopPropagation()}>
            <div className="modal-header">
              <h2>{order.data.order_type} {serial}</h2>
              <button className="close-button" onClick={() => setShowModal(false)}>×</button>
            </div>
            <div className="modal-body">
              {order.paragraphs?.sort((a, b) => a.data.ordinal_sequence - b.data.ordinal_sequence)
                .map((paragraph) => (
                <div 
                  key={paragraph.data.id} 
                  className="paragraph-item"
                  style={{ marginLeft: `${paragraph.data.indent_level * 20}px` }}
                >
                  <h3>{paragraph.data.ordinal_sequence}. {paragraph.data.title}</h3>
                  <p>{paragraph.data.text}</p>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}
    </>
  );
};

export default OrderCard;


