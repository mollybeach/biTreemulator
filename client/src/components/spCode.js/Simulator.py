import { useEffect, useRef} from 'react';
import {sculptToMinimalRenderer} from 'shader-park-core';
//import { glslToMinimalRenderer } from 'shader-park-core';
//import Input from '../Input/Input.jsx';
//import {inputtree} from '../Input/Input'


import './Simulator.scss';

/* eslint-disable */
/*let prepend = () =>{
 let res= `let tree = JSON.parse(\`${inputtree}\`);\n let binaryTree = ${binaryTree} \n binaryTree(tree)`
 return res
}
*/
const Simulator = ({ inputtree }) => { 
  const shadeRef = useRef(null);
  useEffect(() => {
    if (shadeRef.current) {
      const canvas = document.querySelector(".my-canvas");
      sculptToMinimalRenderer(canvas, inputtree);
       //glslToMinimalRenderer(canvas, spCode);
    }
  }, inputtree);

    

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
