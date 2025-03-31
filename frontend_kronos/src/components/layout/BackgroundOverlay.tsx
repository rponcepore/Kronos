import React from "react";
import overlayImage from "/src/assets/rectangle-1.png";
import "../../styles/BackgroundOverlay.css";

const BackgroundOverlay = () => {
  return (
    <div className="background-container">
      <img className="overlay" alt="Overlay" src={overlayImage} />
    </div>
  );
};

export default BackgroundOverlay;
