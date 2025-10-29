import React from 'react';
import './ToggleSwitch.css';

const ToggleSwitch = ({ isOn, handleToggle }) => {
    return (
      <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
        <span>Партия</span>
        <label style={{ position: 'relative', display: 'inline-block', width: 50, height: 24 }}>
          <input
            type="checkbox"
            checked={isOn}
            onChange={handleToggle}
            style={{ opacity: 0, width: 0, height: 0 }}
          />
          <span
            style={{
              position: 'absolute',
              cursor: 'pointer',
              top: 0,
              left: 0,
              right: 0,
              bottom: 0,
              backgroundColor: isOn ? '#4CAF50' : '#ccc',
              transition: '.4s',
              borderRadius: 34,
            }}
          />
          <span
            style={{
              position: 'absolute',
              content: '""',
              height: 18,
              width: 18,
              left: isOn ? 26 : 4,
              bottom: 3,
              backgroundColor: 'white',
              transition: '.4s',
              borderRadius: '50%',
            }}
          />
        </label>
        <span>Плавка</span>
      </div>
    );
  };
  

export default ToggleSwitch;
