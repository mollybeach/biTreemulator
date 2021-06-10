import { useEffect, useState, useRef } from 'react';
import {sculptToMinimalRenderer} from 'shader-park-core';
//import { glslToMinimalRenderer } from 'shader-park-core';
import Input from '../Input/Input.jsx';
import './Simulator.scss';

/* eslint-disable */
let tree ={
  val: 0,
    left: 	{val: 1,
      left: { val: 2, 
                      left: { val: 3, 
                                left: { val: 4, 
                                            left:{ val: 5, left:null, right:null},
                                            right: { val: 6, 
                                                        left:{ val: 7, left:null, right:null},
                                                        right:{ val: 8, left:null, right:null}
                                                      }, 
                                        }          
                            },
                      right: null
            },
          right: { val: 9, 
                      left: { val: 10, left:null, right:null},
                      right: null
              } 
          },
  
    right: {
      val: 11,
            left: { val: 12, 
                      left: null, 
                      right: null
                  },
            right: {
                  val: 13,
                      left: null,
                      right: { val: 14, 
                                  left: { val: 15, 
                                            left:{ val: 16, 
                                                      left:null, 
                                                      right: { val: 17, left:null, right:null},
                                                  }, 
                                            right:null 
                                        }, 
                                  right: { val: 18, 
                                            left:null, 
                                            right: { val: 19, 
                                                        left:{ val: 20, left:null, right:null},
                                                        right:null
                                                      }
                                        },
                            }
                    }
          }
}

function binaryTree() { 
  let gen = getSpace(), sig=0;

  function graph(gen) {
      displace(0.0,0.5,0.0);
      lightDirection(0.0, 1.0, 1.0);
      displace(0.16*gen.x, 0.16*gen.y, abs(abs(gen.y)-abs(gen.x))*0.025*sig);
      color(1.0,1.0,1.0)
      let pos1 = vec3(0.0, 0.0, 0.0), pos2 = vec3( 0.16*sig, 0.16, abs(abs(gen.y)-abs(gen.x))*0.025*-sig), ln=line(pos1, pos2, 0.01);
      color(0,0.0-(gen.y*0.1),0.9-sig)
      sphere(0.075);
      reset();
  }
  function build(tree, gen) {
      graph(gen)
      gen=gen-1;
    if (!tree.left && !tree.right) gen.y=gen.y+1;
    if (tree.left) sig=1,  build(tree.left, gen);
    if (tree.right) sig=-1, gen.x=gen.x+2, build(tree.right, gen);
    
  }
  build(tree, gen);
  }

const Simulator = ({ simulatorList }) => { 
  const [counter, setCounter] = useState(0);
  const shadeRef = useRef(null);
  useEffect(() => {
    if (shadeRef.current) {
      const canvas = document.querySelector(".my-canvas");
      sculptToMinimalRenderer(canvas, binaryTree);
       //glslToMinimalRenderer(canvas, spCode);
    }
  }, [counter]);

  return (
<>
  <div className="simulator">
    <div>
  
      <div className="simulator__inside ">
      
        <div className="simulator__title "> biTreemulator:</div>
        <button onClick={() => setCounter(counter => counter + 1)} className="btn btn--delta" type="submit"><span>Update State</span>
           
            </button>
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





//   window.scrollTo(0, ref.current.offsetTop);




//<img className="simulator__arrow" src={chevronImg} alt="img" />
////import { binaryTree} from './binaryTree';
//import chevronImg from "../../assets/Icons/chevron_right-24px.svg";





  //  let source = spCode.toString();
   // let sourceRes = `let lstp = JSON.parse(\`${simulatorList}\`);\n` + source;
   //console.log(sourceRes);

     /*
      // With a function defined separately
     getData().then((resp) => {
     // sculptToMinimalRenderer(canvas, fullSpCode(JSON.stringify(simulatorList)));
      //sculptToMinimalRenderer(canvas, 'sphere(0.5);');
     */