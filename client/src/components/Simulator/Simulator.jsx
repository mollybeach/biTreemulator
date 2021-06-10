import { useEffect, useRef } from 'react';
import {sculptToMinimalRenderer} from 'shader-park-core';
//import { glslToMinimalRenderer } from 'shader-park-core';
import Input from '../Input/Input.jsx';
import {tree} from './tree'
import {binaryTree} from './binaryTree'
import './Simulator.scss';

/* eslint-disable */
let prepend = () =>{
 let res= `let tree = JSON.parse(\`${tree}\`);\n let binaryTree = ${binaryTree} \n binaryTree(tree)`
 return res
}
const Simulator = ({ tree }) => { 
  const shadeRef = useRef(null);

  useEffect(() => {
    if (shadeRef.current) {
      const canvas = document.querySelector(".my-canvas");
      sculptToMinimalRenderer(canvas, prepend());
       //glslToMinimalRenderer(canvas, spCode);
    }
  }, tree);

  return (
<>
  <div className="simulator">
    <div>
      <div className="simulator__inside ">
        <div className="simulator__title "> biTreemulator:</div>
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
