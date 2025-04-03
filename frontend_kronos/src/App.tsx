import React, { useState, useEffect } from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import NavBar from "./components/Navbar.tsx"; // Adjust the path to the correct location of Navbar
import "./App.css"; // Import your global styles
import "./styles/darkMode.css";
import PlansPage from "./pages/PlansPage";
import PlansOverview from "./components/Plans/PlansOverview.tsx";
import PlanDetails from "./components/Plans/PlansDetails.tsx";
import "./styles/plans.css";


// Placeholder components for the pages
const Overview = () => <div>Overview Page</div>;
const Dashboard = () => <div>Dashboard Page</div>;
const TaskManagement = () => <div>Task Management Page</div>;
const Calendar = () => <div>Calendar Page</div>;
const Analytics = () => <div>Analytics Page</div>;
const Search = () => <div>Search Page</div>;
const Settings = () => <div>Settings Page</div>;

const App = () => {
  const [darkMode, setDarkMode] = useState(() => {
    // Get initial state from localStorage or system preference
    const saved = localStorage.getItem('darkMode');
    if (saved !== null) {
      return JSON.parse(saved);
    }
    return window.matchMedia('(prefers-color-scheme: dark)').matches;
  });

  // Update body class and localStorage when darkMode changes
  useEffect(() => {
    if (darkMode) {
      document.body.classList.add('dark-mode');
    } else {
      document.body.classList.remove('dark-mode');
    }
    localStorage.setItem('darkMode', JSON.stringify(darkMode));
  }, [darkMode]);

  const toggleDarkMode = () => {
    setDarkMode(!darkMode);
  };

  return (
    <Router>
      <NavBar darkMode={darkMode} onToggleDarkMode={toggleDarkMode} />
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
};

export default App;
