import { useEffect, useRef} from 'react';
import {sculptToMinimalRenderer} from 'shader-park-core';
//import { glslToMinimalRenderer } from 'shader-park-core';
import './Simulator.scss';

const Simulator = ({ inputtree }) => { 
  const shadeRef = useRef(null);
  useEffect(() => {
    if (shadeRef.current) {
      const canvas = document.querySelector(".my-canvas");
      sculptToMinimalRenderer(canvas, inputtree);
       //glslToMinimalRenderer(canvas, spCode);
    }
  }, [inputtree]);
    
  return (
<>
        <canvas className="my-canvas"></canvas>
        <iframe title="miframie" ref={shadeRef}>
          <body className="removeAdditionaFrame"></body>
        </iframe>
</>
  );
};

export default Simulator;
