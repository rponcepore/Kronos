import React from "react";
import { Plan } from "../types/Plan";
import { Order } from "../types/Order";
import { orders } from "../dummy_data/dummy_orders";

interface PlanDetailsProps {
  plan: Plan;
  orders: Order[];
  goBack: () => void;
}

const PlanDetails: React.FC<PlanDetailsProps> = ({ plan, orders, goBack }) => {
  /*
  const childPlans = allPlans.filter(
    (p) => p.number.startsWith(plan.number) && p.id !== plan.id
  );
  */
  const childOrders = getOrdersForPlan(plan, orders);
  return (
    <div className="plan-details">
      {/* Back Button Aligned Left */}
      <div className="back-button-wrapper">
        <button className="back-button" onClick={goBack}>← Back</button>
      </div>

      {/* Main Plan Info */}
      <h1>
        {/*<span className={`plan-type ${plan.type.toLowerCase()}`}>{plan.type}</span>*/} 
        Plan {plan.fiscal_year}-{plan.serial_number}: {plan.name}
      </h1>
      {/*
      <p><strong>Published:</strong> {plan.published}</p>
      <p><strong>Expires:</strong> {plan.expires}</p>
      <p><strong>Tasked By:</strong> {plan.taskedBy}</p>
      <p><strong>Tasked To:</strong> {plan.taskedTo}</p>
      <p><strong>Mission:</strong> {plan.mission}</p>
      */
      }
      {/* Related Orders Section */}
      {childOrders.length > 0 && (
  <div className="related-orders">
    <h2>Related Orders</h2>
    <div className="related-orders-container">
      {childOrders.map((child) => (
        <div key={child.id} className="order-card">
          <span className={`order-type ${child.order_type.toLowerCase()}`}>{child.order_type} {child.serial_number} </span>
          {/*<h3>{child.number}</h3>
          <p><strong>Published:</strong> {child.is_published}</p>
          {/*
          <p><strong>Expires:</strong> {child.expires}</p>
          <p><strong>Tasked To:</strong> {child.taskedTo}</p>
          <p><strong>Mission:</strong> {child.mission}</p>
          */}
        </div>
      ))}
    </div>
  </div>
)}

    </div>
  );
};

export default PlanDetails;

function getOrdersForPlan(plan: Plan, orders: Order[]): Order[] {
  return orders.filter(order => order.parent_plan === plan.id);
}

