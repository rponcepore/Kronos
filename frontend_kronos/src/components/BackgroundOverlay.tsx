import React from "react";
import backgroundImage from "/src/assets/image-1.png";
import overlayImage from "/src/assets/rectangle-1.svg";
import "../styles/BackgroundOverlay.css";

const BackgroundOverlay = () => {
  return (
    <div className="background-container">
      <img className="background" alt="Background" src={backgroundImage} />
      <img className="overlay" alt="Overlay" src={overlayImage} />
    </div>
  );
};

export default BackgroundOverlay;
