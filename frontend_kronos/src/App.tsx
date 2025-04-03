import React, { useState, useEffect } from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import NavBar from "./components/Navbar.tsx"; // Adjust the path to the correct location of Navbar
import "./App.css"; // Import your global styles
import "./styles/theme.css"; // Import global theme
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

function App() {
  const [isDarkMode, setIsDarkMode] = useState(() => {
    // Check if user has a saved preference
    const savedTheme = localStorage.getItem('theme');
    // Check system preference if no saved preference
    if (!savedTheme) {
      return window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    return savedTheme === 'dark';
  });

  useEffect(() => {
    // Update the data-theme attribute when dark mode changes
    document.documentElement.setAttribute('data-theme', isDarkMode ? 'dark' : 'light');
    // Save preference to localStorage
    localStorage.setItem('theme', isDarkMode ? 'dark' : 'light');
  }, [isDarkMode]);

  // Listen for system theme changes
  useEffect(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = (e: MediaQueryListEvent) => {
      if (!localStorage.getItem('theme')) {
        setIsDarkMode(e.matches);
      }
    };
    
    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, []);

  const toggleDarkMode = () => {
    setIsDarkMode(!isDarkMode);
  };

  return (
    <Router>
      <div className="app-container">
        <NavBar />
        <button 
          onClick={toggleDarkMode}
          style={{
            position: 'fixed',
            bottom: '20px',
            right: '20px',
            padding: '10px',
            borderRadius: '50%',
            border: 'none',
            backgroundColor: isDarkMode ? '#fff' : '#333',
            color: isDarkMode ? '#333' : '#fff',
            cursor: 'pointer',
            width: '40px',
            height: '40px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            boxShadow: '0 2px 5px rgba(0,0,0,0.2)',
            zIndex: 1000,
          }}
        >
          {isDarkMode ? '☀️' : '🌙'}
        </button>
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
      </div>
    </Router>
  );
}

export default App;
