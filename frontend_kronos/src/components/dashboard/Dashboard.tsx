import React from "react";
import Navbar from "../layout/Navbar";
import Notifications from "./Notifications";
import HighPriorityTasks from "./HighPriorityTasks";
import MissionReadinessChart from "./MissionReadinessChart";
import WeeklyActivityChart from "./WeeklyActivityChart";
import BackgroundOverlay from "../layout/BackgroundOverlay"; // Import background component

const Dashboard = () => (
  <div className="p-4 bg-gray-900 text-white min-h-screen relative">
    <BackgroundOverlay /> {/* Background always presentt */}
    <Navbar />
    <div className="p-4 grid grid-cols-2 gap-4 relative">
      <Notifications />
      <HighPriorityTasks />
      <MissionReadinessChart />
      <WeeklyActivityChart />
    </div>
  </div>
);

export default Dashboard;
