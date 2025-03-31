// React & State Hooks
import React, { useState } from "react";

// Types for data props
import { KronosOrderSummary } from "../../types/frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../../types/frontend_types/ParagraphSummary";

// CSS styling for this component
import "../../styles/OrderDetails.css";

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
    <div className="max-w-4xl mx-auto p-6">
      <button 
        onClick={goBack}
        className="mb-6 flex items-center text-blue-600 hover:text-blue-800"
      >
        <span className="mr-2">←</span> Back
      </button>

      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">
          {order.data.order_type} {order.data.serial_number}
        </h1>
        <p className="text-gray-600">
          <span className="font-medium">Published:</span> {order.data.is_published ? "Yes" : "No"}
        </p>
        {order.data.derived_from && (
          <p className="text-gray-600">
            <span className="font-medium">Derived From:</span> {order.data.derived_from}
          </p>
        )}
      </div>

      <div className="space-y-6">
        {relevantParagraphs.map(p => (
          <div
            key={p.data.id}
            className={`p-4 rounded-lg border ${
              isDraft ? 'hover:bg-gray-50 cursor-pointer' : ''
            }`}
            onClick={() => handleParagraphClick(p.data.id, p.data.text)}
          >
            <h3 className="text-xl font-semibold mb-2">{p.data.title}</h3>

            {editingId === p.data.id ? (
              <div className="space-y-2">
                <textarea
                  className="w-full p-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  value={paragraphDrafts[p.data.id]}
                  onChange={(e) => handleChange(p.data.id, e.target.value)}
                  rows={4}
                />
                <button 
                  onClick={() => handleSave(p.data.id)}
                  className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
                >
                  Save
                </button>
              </div>
            ) : (
              <p className="text-gray-700 whitespace-pre-wrap">{p.data.text}</p>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

export default OrderDetails;

