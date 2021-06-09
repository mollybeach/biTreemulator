import { useEffect, useRef } from 'react';
//import { glslToMinimalRenderer } from 'shader-park-core';
import {sculptToMinimalRenderer} from 'shader-park-core';

import { binaryTree} from './binaryTree.js';
/****************USE HOOKS TO LOAD SHADER ***********************/
const GS = ({ treeList }) => { 
  const shadeRef = useRef(null);
  useEffect(() => {
    if (shadeRef.current) {
      /******USE JS OR GLSL CODE FOR SHADER */
      const canvas = document.querySelector(".my-canvas");
     //glslToMinimalRenderer(canvas, spCode);
     sculptToMinimalRenderer(canvas, binaryTree);
  



  //  let source = spCode.toString();
   // let sourceRes = `let lstp = JSON.parse(\`${treeList}\`);\n` + source;
   //console.log(sourceRes);

     /*
      // With a function defined separately
     getData().then((resp) => {
     // sculptToMinimalRenderer(canvas, fullSpCode(JSON.stringify(treeList)));
      //sculptToMinimalRenderer(canvas, 'sphere(0.5);');
     */
     

    
    }
  }, []);

  return (
    <div>
      <canvas className="my-canvas"></canvas>
      <iframe title="miframie" ref={shadeRef}>
    <body className="removeAdditionaFrame"></body>
      </iframe>
    
    </div>
  );
};

export default GS;
  
