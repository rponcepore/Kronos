import React from "react";
import { Link, useLocation } from "react-router-dom"; // useLocation determines the active route
import "../styles/NavBar.css"; // CSS File 
import logo from "../assets/KronosLogo.png"; // Import the logo image

const NavBar = () => {
  const location = useLocation(); // Hook to get the current location/path
  
  // Helper function to check if the link is active
  const isActive = (path: string) => location.pathname === path;

  return (
    <nav className="navbar">
              <img src={logo} alt="Logo" className="logo" /> {/* Logo on the left */}

      <ul className="nav-links">
        <li className={isActive("/") ? "active" : ""}>
          <Link to="/">Overview</Link>
        </li>
        <li className={isActive("/dashboard") ? "active" : ""}>
          <Link to="/dashboard">Dashboard</Link>
        </li>
        <li className={isActive("/plans") ? "active" : ""}>
          <Link to="/plans">Plans</Link>
        </li>
        <li className={isActive("/task-management") ? "active" : ""}>
          <Link to="/task-management">Task Management</Link>
        </li>
        <li className={isActive("/calendar") ? "active" : ""}>
          <Link to="/calendar">Calendar</Link>
        </li>
        <li className={isActive("/analytics") ? "active" : ""}>
          <Link to="/analytics">Analytics</Link>
        </li>
        <li className={isActive("/search") ? "active" : ""}>
          <Link to="/search">Search</Link>
        </li>
        <li className={isActive("/settings") ? "active" : ""}>
          <Link to="/settings">Settings</Link>
        </li>
      </ul>
    </nav>
  );
};

export default NavBar;

