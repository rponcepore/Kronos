import React, { useState, useRef } from "react";
import { KronosOrderSummary } from "../types/frontend_types/KronosOrderSummary";
import { ParagraphSummary } from "../types/frontend_types/ParagraphSummary";
import { getOrderSerialDisplay } from "../helper_methods/format";
import "../styles/orderCard.css";

// Context menu interface
interface ContextMenuState {
  show: boolean;
  x: number;
  y: number;
  paragraphId: number | null;
}

interface ConfirmDialogState {
  show: boolean;
  paragraphId: number | null;
  originalText: string;
  originalTitle: string;
}

// Props interface for the OrderCard component
interface OrderCardProps {
  order: KronosOrderSummary;
  parentPlanFiscalYear: number;
  selectOrder: (order: KronosOrderSummary) => void;
  paragraphs: ParagraphSummary[] | null;
  onUpdateParagraph?: (paragraphId: number, newText: string, newTitle: string) => void;
  onDeleteParagraph?: (paragraphId: number) => void;
  onAddParagraph?: (beforeId: number, indentLevel: number, ordinalSequence: number, parentParagraph: number | null) => void;
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
  onUpdateParagraph,
  onDeleteParagraph,
  onAddParagraph,
}) => {
  const [showModal, setShowModal] = useState(false);
  const [selectedParagraphId, setSelectedParagraphId] = useState<number | null>(null);
  const [editingParagraph, setEditingParagraph] = useState<number | null>(null);
  const [editText, setEditText] = useState("");
  const [editTitle, setEditTitle] = useState("");
  const [newParagraphPosition, setNewParagraphPosition] = useState<{
    beforeId: number, 
    indentLevel: number,
    ordinalSequence: number,
    parentParagraph: number | null,
    paragraph: ParagraphSummary
  } | null>(null);
  const [confirmDiscard, setConfirmDiscard] = useState<ConfirmDialogState>({
    show: false,
    paragraphId: null,
    originalText: "",
    originalTitle: "",
  });

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

  const handleClickOutside = () => {
    setSelectedParagraphId(null);
  };

  const getParagraphNumbers = (
    paragraphs: ParagraphSummary[]
  ): Map<number, string> => {
    const numberingMap = new Map<number, string>();

    for (const paragraph of paragraphs) {
      const { id, ordinal_sequence } = paragraph.data;
      numberingMap.set(id, ordinal_sequence.toString());
    }

    return numberingMap;
  };

  // Pre-compute paragraph numbers once
  const paragraphNumbers = getParagraphNumbers(order.paragraphs || []);

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
              {order.paragraphs?.sort((a, b) => a.data.id - b.data.id)
                .map((paragraph) => {
                const level = paragraph.data.indent_level % 4;
                let prefix;
                switch (level) {
                  case 0: prefix = `${paragraph.data.ordinal_sequence}. `; break;
                  case 1: prefix = `${String.fromCharCode(96 + paragraph.data.ordinal_sequence)}. `; break;
                  case 2: prefix = `(${paragraph.data.ordinal_sequence}) `; break;
                  case 3: prefix = `(${String.fromCharCode(96 + paragraph.data.ordinal_sequence)}) `; break;
                }
                
                return (
                  <React.Fragment key={paragraph.data.id}>
                    <div 
                      className="paragraph-item"
                      style={{ "--indent-level": paragraph.data.indent_level } as React.CSSProperties}
                    >
                      {editingParagraph === paragraph.data.id ? (
                        <div className="paragraph-edit">
                          <div className="edit-header">
                            <span className="paragraph-number">{prefix}</span>
                            <input
                              type="text"
                              value={editTitle}
                              onChange={(e) => setEditTitle(e.target.value)}
                              className="edit-title"
                              placeholder="Enter title..."
                            />
                          </div>
                          <textarea
                            value={editText}
                            onChange={(e) => setEditText(e.target.value)}
                            className="edit-text"
                            placeholder="Enter paragraph text..."
                          />
                          <div className="edit-actions">
                            <button 
                              className="save-button"
                              onClick={() => setEditingParagraph(null)}
                            >
                              Save
                            </button>
                            <button 
                              className="discard-button"
                              onClick={() => {
                                setConfirmDiscard({
                                  show: true,
                                  paragraphId: paragraph.data.id,
                                  originalText: paragraph.data.text || '',
                                  originalTitle: paragraph.data.title || ''
                                });
                              }}
                            >
                              Discard Changes
                            </button>
                          </div>
                        </div>
                      ) : (
                        <>
                          <div 
                            className="paragraph-content"
                            onClick={() => setSelectedParagraphId(selectedParagraphId === paragraph.data.id ? null : paragraph.data.id)}
                          >
                            <h3>{prefix}{paragraph.data.title}</h3>
                            <p>{paragraph.data.text}</p>
                          </div>
                          {selectedParagraphId === paragraph.data.id && paragraph.data.indent_level > 0 && (
                            <div className="paragraph-menu">
                              <button
                                className="menu-button"
                                onClick={() => {
                                  setEditingParagraph(paragraph.data.id);
                                  setEditText(paragraph.data.text || '');
                                  setEditTitle(paragraph.data.title || '');
                                  setSelectedParagraphId(null);
                                }}
                              >
                                Edit
                              </button>
                              <button
                                className="menu-button"
                                onClick={() => {
                                  const nextOrdinalSequence = paragraph.data.ordinal_sequence;
                                  const newParagraph: ParagraphSummary = {
                                    data: {
                                      id: paragraph.data.id - 1, // Temporary ID
                                      order: paragraph.data.order,
                                      parent_paragraph: paragraph.data.parent_paragraph,
                                      ordinal_sequence: nextOrdinalSequence,
                                      indent_level: paragraph.data.indent_level,
                                      is_major: paragraph.data.is_major,
                                      title: '',
                                      text: ''
                                    },
                                    subParagraphs: []
                                  };
                                  setNewParagraphPosition({
                                    beforeId: paragraph.data.id,
                                    indentLevel: paragraph.data.indent_level,
                                    ordinalSequence: nextOrdinalSequence,
                                    parentParagraph: paragraph.data.parent_paragraph,
                                    paragraph: newParagraph
                                  });
                                  setEditText('');
                                  setEditTitle('');
                                  setSelectedParagraphId(null);
                                }}
                              >
                                Add Above
                              </button>
                              <button
                                className="menu-button"
                                onClick={() => {
                                  const nextOrdinalSequence = paragraph.data.ordinal_sequence + 1;
                                  const newParagraph: ParagraphSummary = {
                                    data: {
                                      id: paragraph.data.id + 1, // Temporary ID
                                      order: paragraph.data.order,
                                      parent_paragraph: paragraph.data.parent_paragraph,
                                      ordinal_sequence: nextOrdinalSequence,
                                      indent_level: paragraph.data.indent_level,
                                      is_major: paragraph.data.is_major,
                                      title: '',
                                      text: ''
                                    },
                                    subParagraphs: []
                                  };
                                  setNewParagraphPosition({
                                    beforeId: paragraph.data.id + 1,
                                    indentLevel: paragraph.data.indent_level,
                                    ordinalSequence: nextOrdinalSequence,
                                    parentParagraph: paragraph.data.parent_paragraph,
                                    paragraph: newParagraph
                                  });
                                  setEditText('');
                                  setEditTitle('');
                                  setSelectedParagraphId(null);
                                }}
                              >
                                Add Below
                              </button>
                              <button
                                className="menu-button"
                                onClick={() => {
                                  const nextOrdinalSequence = 1; // Start with 1 for subparagraphs
                                  const newParagraph: ParagraphSummary = {
                                    data: {
                                      id: paragraph.data.id + 1, // Temporary ID
                                      order: paragraph.data.order,
                                      parent_paragraph: paragraph.data.id,
                                      ordinal_sequence: nextOrdinalSequence,
                                      indent_level: paragraph.data.indent_level + 1,
                                      is_major: paragraph.data.is_major,
                                      title: '',
                                      text: ''
                                    },
                                    subParagraphs: []
                                  };
                                  setNewParagraphPosition({
                                    beforeId: paragraph.data.id + 1,
                                    indentLevel: paragraph.data.indent_level + 1,
                                    ordinalSequence: nextOrdinalSequence,
                                    parentParagraph: paragraph.data.id,
                                    paragraph: newParagraph
                                  });
                                  setEditText('');
                                  setEditTitle('');
                                  setSelectedParagraphId(null);
                                }}
                              >
                                Add Subparagraph
                              </button>
                              <button
                                className="menu-button delete"
                                onClick={() => {
                                  if (onDeleteParagraph) {
                                    onDeleteParagraph(paragraph.data.id);
                                  }
                                  setSelectedParagraphId(null);
                                }}
                              >
                                Delete
                              </button>
                            </div>
                          )}
                        </>
                      )}
                    </div>
                    {newParagraphPosition && newParagraphPosition.beforeId === paragraph.data.id + 1 && (
                      <div 
                        className="paragraph-item"
                        style={{ "--indent-level": newParagraphPosition.indentLevel } as React.CSSProperties}
                      >
                        <div className="paragraph-edit">
                          <div className="edit-header">
                            <span className="paragraph-number">
                              {(() => {
                                const level = newParagraphPosition.indentLevel % 4;
                                switch (level) {
                                  case 0: return `${newParagraphPosition.ordinalSequence}. `;
                                  case 1: return `${String.fromCharCode(96 + newParagraphPosition.ordinalSequence)}. `;
                                  case 2: return `(${newParagraphPosition.ordinalSequence}) `;
                                  case 3: return `(${String.fromCharCode(96 + newParagraphPosition.ordinalSequence)}) `;
                                  default: return '';
                                }
                              })()}
                            </span>
                            <input
                              type="text"
                              value={editTitle}
                              onChange={(e) => setEditTitle(e.target.value)}
                              className="edit-title"
                              placeholder="Enter title..."
                            />
                          </div>
                          <textarea
                            value={editText}
                            onChange={(e) => setEditText(e.target.value)}
                            className="edit-text"
                            placeholder="Enter paragraph text..."
                          />
                          <div className="edit-actions">
                            <button 
                              className="save-button"
                              onClick={() => {
                                if (onAddParagraph) {
                                  onAddParagraph(
                                    newParagraphPosition.beforeId, 
                                    newParagraphPosition.indentLevel,
                                    newParagraphPosition.ordinalSequence,
                                    newParagraphPosition.parentParagraph
                                  );
                                }
                                setNewParagraphPosition(null);
                                setEditText('');
                                setEditTitle('');
                              }}
                            >
                              Save
                            </button>
                            <button 
                              className="discard-button"
                              onClick={() => {
                                setNewParagraphPosition(null);
                                setEditText('');
                                setEditTitle('');
                              }}
                            >
                              Cancel
                            </button>
                          </div>
                        </div>
                      </div>
                    )}
                  </React.Fragment>
                );
              })}
            </div>
          </div>
        </div>
      )}

      {/* Confirmation Dialog */}
      {confirmDiscard.show && (
        <div className="modal-overlay" onClick={(e) => e.stopPropagation()}>
          <div 
            className="confirm-dialog"
            style={{
              position: 'fixed',
              top: '50%',
              left: '50%',
              transform: 'translate(-50%, -50%)',
              zIndex: 2000,
              background: 'white',
              padding: '20px',
              borderRadius: '8px',
              boxShadow: '0 2px 10px rgba(0,0,0,0.1)',
              width: '400px',
              textAlign: 'center'
            }}
          >
            <h3 style={{ marginTop: 0 }}>Discard Changes?</h3>
            <p>Are you sure you want to discard your changes?</p>
            <div style={{ display: 'flex', justifyContent: 'center', gap: '10px', marginTop: '20px' }}>
              <button
                className="discard-button"
                onClick={() => {
                  setEditingParagraph(null);
                  setEditText(confirmDiscard.originalText);
                  setEditTitle(confirmDiscard.originalTitle);
                  setConfirmDiscard({ show: false, paragraphId: null, originalText: '', originalTitle: '' });
                }}
              >
                Discard
              </button>
              <button
                className="save-button"
                onClick={() => setConfirmDiscard({ show: false, paragraphId: null, originalText: '', originalTitle: '' })}
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

export default OrderCard;


