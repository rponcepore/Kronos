// React & State Hooks
import React, { useState } from "react";

// Types for data props
import { KronosOrderSummary } from "../types/frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../types/frontend_types/ParagraphSummary";

// CSS styling for this component
import "../styles/OrderDetails.css";

// --------------------------------------------
// Props for OrderDetails component
// --------------------------------------------
interface OrderDetailsProps {
  order: KronosOrderSummary;
  allParagraphs: ParagraphSummary[];
  goBack: () => void;
}

// --------------------------------------------
// OrderDetails Component
// Displays a full order and allows editing of its paragraphs if it's in draft mode
// --------------------------------------------
const OrderDetails: React.FC<OrderDetailsProps> = ({
  order,
  allParagraphs,
  goBack,
}) => {
  // Tracks the currently editing paragraph (if any)
  const [editingId, setEditingId] = useState<number | null>(null);

  // Stores the paragraph draft edits keyed by paragraph ID
  const [paragraphDrafts, setParagraphDrafts] = useState<Record<number, string>>({});

  // Determine if the order is still in draft mode (not published)
  const isDraft = !order.data.is_published;

  // Filter for top-level paragraphs belonging to this order and sort by sequence
  const relevantParagraphs = allParagraphs
    .filter(p => p.data.order === order.data.id && p.data.parent_paragraph === null)
    .sort((a, b) => a.data.ordinal_sequence - b.data.ordinal_sequence);

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
        {order.data.order_type} {order.data.serial_number}
      </h1>
      <p><strong>Published:</strong> {order.data.is_published ? "Yes" : "No"}</p>
      {order.data.derived_from && (
        <p><strong>Derived From:</strong> {order.data.derived_from}</p>
      )}

      <hr />

      {/* Render each top-level paragraph */}
      {relevantParagraphs.map(p => (
        <div
          key={p.data.id}
          className="paragraph-block"
          onClick={() => handleParagraphClick(p.data.id, p.data.text)}
        >
          <h3>{p.data.title}</h3>

          {editingId === p.data.id ? (
            <>
              <textarea
                className="paragraph-edit-area"
                value={paragraphDrafts[p.data.id]}
                onChange={(e) => handleChange(p.data.id, e.target.value)}
              />
              <button className="save-btn" onClick={() => handleSave(p.data.id)}>
                Save
              </button>
            </>
          ) : (
            <p>{p.data.text}</p>
          )}
        </div>
      ))}
    </div>
  );
};

export default OrderDetails;

