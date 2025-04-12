import React, { useState } from "react";
import { KronosApiMethod } from "../../types/networking_types/KronosApiMethodEnums";
import { kronosApiCall } from "../../helper_methods/ApiCall";
import { KronosRequest } from "../../types/networking_types/KronosRequest";
import { PlanRequest } from "../../types/networking_types/PlanRequest";
import { Classification } from "../../types/enums/Classification";

interface NewPlanModalProps {
  onClose: () => void;
  onSuccess: () => void;
}

const NewPlanModal: React.FC<NewPlanModalProps> = ({ onClose, onSuccess }) => {
  const [planName, setPlanName] = useState("");
  const [uic, setUic] = useState("");

  const handleSubmit = async () => {
    const planReq: PlanRequest = {
      plan_id: null,
      plan_name: planName,
      classification: Classification.CUI, // default as per your backend
    };

    const req: KronosRequest = {
      api_method: KronosApiMethod.create_plan,
      uic,
      plan_request: planReq,
      order_request: null,
      paragraph_request: null,
      task_request: null,
    };

    try {
      await kronosApiCall(req);
      onSuccess(); // e.g., refresh plans
      onClose();   // close modal
    } catch (err) {
      console.error("Error creating plan:", err);
    }
  };

  return (
      <div className="new-plan-modal-overlay">
          <div className="new-plan-modal-content">
        <h2>Create New Plan</h2>
        <label>Plan Name</label>
        <input
          className="control-input"
          type="text"
          value={planName}
          onChange={(e) => setPlanName(e.target.value)}
          placeholder="Enter plan name"
        />
        <label>UIC</label>
        <input
          className="control-input"
          type="text"
          value={uic}
          onChange={(e) => setUic(e.target.value)}
          placeholder="Enter UIC if making a plan for another unit"
        />

        <div className="modal-actions">
          <button className="control-btn" onClick={onClose}>Cancel</button>
          <button className="control-btn" onClick={handleSubmit}>Save</button>
        </div>
      </div>
    </div>
  );
};

export default NewPlanModal;
