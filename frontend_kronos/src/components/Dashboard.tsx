import React from "react";
import Navbar from "./Navbar";
import Notifications from "./Notifications";
import HighPriorityTasks from "./HighPriorityTasks";
import MissionReadinessChart from "./missionReadinessChart";
import WeeklyActivityChart from "./WeeklyActivityChart";

const Dashboard = () => (
  <div className="p-4 bg-gray-900 text-white min-h-screen">
    <Navbar />
    <div className="p-4 grid grid-cols-2 gap-4">
      <Notifications />
      <HighPriorityTasks />
      <MissionReadinessChart />
      <WeeklyActivityChart />
    </div>
  </div>
);

export default Dashboard;

