import React from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import { Navbar } from "./components";
import "./App.css"; // Import your global styles
import PlansPage from "./pages/PlansPage";
import "./styles/plans.css";

// Placeholder components for the pages
const Overview = () => <div>Overview Page</div>;
const Dashboard = () => <div>Dashboard Page</div>;
const TaskManagement = () => <div>Task Management Page</div>;
const Calendar = () => <div>Calendar Page</div>;
const Analytics = () => <div>Analytics Page</div>;
const Search = () => <div>Search Page</div>;
const Settings = () => <div>Settings Page</div>;

function App() {
  return (
    <Router>
      <Navbar />
      <Routes>
        <Route path="/" element={<Overview />} />
        <Route path="/dashboard" element={<Dashboard />} />
        <Route path="/plans" element={<PlansPage />} />
        <Route path="/task-management" element={<TaskManagement />} />
        <Route path="/calendar" element={<Calendar />} />
        <Route path="/analytics" element={<Analytics />} />
        <Route path="/search" element={<Search />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </Router>
  );
}

export default App;
