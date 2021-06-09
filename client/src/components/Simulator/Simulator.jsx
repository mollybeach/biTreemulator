
import { useEffect, useRef } from 'react';
//import { glslToMinimalRenderer } from 'shader-park-core';
import {sculptToMinimalRenderer} from 'shader-park-core';

import { binaryTree} from './binaryTree';
import chevronImg from "../../assets/Icons/chevron_right-24px.svg";
import Input from '../Input/Input.jsx';
import './Simulator.scss';
/****************USE HOOKS TO LOAD SHADER ***********************/
const Simulator = ({ simulatorList }) => { 
  const shadeRef = useRef(null);
  useEffect(() => {
    if (shadeRef.current) {
      /******USE JS OR GLSL CODE FOR SHADER */
      const canvas = document.querySelector(".my-canvas");
     //glslToMinimalRenderer(canvas, spCode);
     sculptToMinimalRenderer(canvas, binaryTree);
  



  //  let source = spCode.toString();
   // let sourceRes = `let lstp = JSON.parse(\`${simulatorList}\`);\n` + source;
   //console.log(sourceRes);

     /*
      // With a function defined separately
     getData().then((resp) => {
     // sculptToMinimalRenderer(canvas, fullSpCode(JSON.stringify(simulatorList)));
      //sculptToMinimalRenderer(canvas, 'sphere(0.5);');
     */
     

    
    }
  }, []);

  return (
    <>
    <div className="simulator">
    <div>
    <div className='simulator__inside '> 
    <div className="simulator__title "> biTreemulator:
            <img className="simulator__arrow" src={chevronImg} alt="img" />
      </div>
          <Input />
      <canvas className="my-canvas"></canvas>
      <iframe title="miframie" ref={shadeRef}>
    <body className="removeAdditionaFrame"></body>
      </iframe>
    
    </div>

    </div>
    </div>
      

      </> 
  );
};

export default Simulator;