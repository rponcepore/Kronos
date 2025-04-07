import React from "react";

const Notifications = () => (
  <div className="bg-gray-200 p-4 rounded-lg">
    <h2 className="font-bold">Notifications</h2>
    <ul>
      <li>[URGENT] Overdue Task: Alpha Troop has not completed Task #274</li>
      <li>Task Reassignment Alert: Task #187 reassigned</li>
      <li>[Weather Alert] Severe weather expected in AO</li>
    </ul>
  </div>
);

export default Notifications;