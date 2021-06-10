
import React, { useState } from "react";
import './Input.scss'
import {tree, binaryTree} from './spCode'
import Simulator from "../Simulator/Simulator";
const Input = ({props }) => { 
    const [inputValue, setValue] = useState(tree);
  let prepend = (inputValue) =>{
    let res= `let tree = JSON.parse(\`${inputValue}\`);\n let binaryTree = ${binaryTree} \n binaryTree(tree)`
    return res
    }
      return(
        <>
      <div className="input">
        <div className="input__container">
          <h1 className="input__intro">Binary Tree Visualization </h1>
          <div className="input__box">
            <textarea
              className="input__input-is-medium"
              type="text"
              id="input"
              value={inputValue}
              onChange={(e) => setValue(e.target.value)}
              rows={5}
              cols={5}
            />
              <p className="input__input-value">The value of the input is: <span className="input__highlight">{inputValue}</span></p>
          </div>
        </div>
      <Simulator inputtree={prepend(inputValue)}/>
        </div>
      </>    
      );
    
  }
  export default Input;

















  //  <label className="label">Enter Binary Input</label>
  //    <p className="input-value">The value of the input is: <span className="highlight">{this.state.value}</span></p>
/*    const handleChange = e => {
      this.setState({value: e.target.value});
    };*/

    //   const [inputValue] = useState([{ text: 'Learn Hooks' }]);[inputValue]= useState(inputValue) {inputValue.map(inputValue => <div>{inputValue.text}</div>)}