// React & State Hooks
import React, { useState } from "react";

// Types for data props
import { KronosOrder } from "../types/KronosOrder";
import { Paragraph } from "../types/Paragraph";
import { Plan } from "../types/Plan";

// CSS styling for this component
import "../styles/OrderDetails.css";

// --------------------------------------------
// Props for OrderDetails component
// --------------------------------------------
interface OrderDetailsProps {
  order: KronosOrder;          // The order being displayed (OPORD, WARNO, FRAGO)
  parentPlan: Plan;            // The plan this order belongs to
  paragraphs: Paragraph[];     // All paragraphs related to this order
  goBack: () => void;          // Callback function to return to the previous screen
}

// --------------------------------------------
// OrderDetails Component
// Displays a full order and allows editing of its paragraphs if it’s in draft mode
// --------------------------------------------
const OrderDetails: React.FC<OrderDetailsProps> = ({
  order,
  parentPlan,
  paragraphs,
  goBack,
}) => {
  // Tracks the currently editing paragraph (if any)
  const [editingId, setEditingId] = useState<number | null>(null);

  // Stores the paragraph draft edits keyed by paragraph ID
  const [paragraphDrafts, setParagraphDrafts] = useState<Record<number, string>>({});

  // Determine if the order is still in draft mode (not published)
  const isDraft = !order.is_published;

  // Filter for top-level paragraphs belonging to this order and sort by sequence
  const relevantParagraphs = paragraphs
    .filter(p => p.order === order.id && p.parent_paragraph === null)
    .sort((a, b) => a.ordinal_sequence - b.ordinal_sequence);

  // Called when a paragraph is clicked (editable only in draft)
  const handleParagraphClick = (id: number, currentText: string) => {
    if (isDraft) {
      setEditingId(id);
      setParagraphDrafts(prev => ({ ...prev, [id]: currentText }));
    }
  };

  // Handles updates to paragraph draft text
  const handleChange = (id: number, value: string) => {
    setParagraphDrafts(prev => ({ ...prev, [id]: value }));
  };

  // Called when user clicks "Save" — you would later hook this to an API call
  const handleSave = (id: number) => {
    // ✅ Here you'd send the updated paragraph to the backend
    setEditingId(null);  // Exit editing mode
  };

  return (
    <div className="order-details-container">
      {/* Back button */}
      <button className="back-button" onClick={goBack}>
        ← Back
      </button>

      {/* Order heading and metadata */}
      <h1>
        {order.order_type} {parentPlan.serial_number}-{order.serial_number}
      </h1>
      <p><strong>Parent Plan:</strong> {parentPlan.name}</p>
      <p><strong>Published:</strong> {order.is_published ? "Yes" : "No"}</p>
      {order.derived_from && (
        <p><strong>Derived From:</strong> {order.derived_from}</p>
      )}

      <hr />

      {/* Render each top-level paragraph */}
      {relevantParagraphs.map(p => (
        <div
          key={p.id}
          className="paragraph-block"
          onClick={() => handleParagraphClick(p.id, p.text)}
        >
          <h3>{p.title}</h3>

          {editingId === p.id ? (
            <>
              <textarea
                className="paragraph-edit-area"
                value={paragraphDrafts[p.id]}
                onChange={(e) => handleChange(p.id, e.target.value)}
              />
              <button className="save-btn" onClick={() => handleSave(p.id)}>
                Save
              </button>
            </>
          ) : (
            <p>{p.text}</p>
          )}
        </div>
      ))}
    </div>
  );
};

export default OrderDetails;

