
import React, { useState } from "react";
import './Input.scss'
import {tree, binaryTree, jtree} from './spCode'
//import Simulator from "../Simulator/Simulator";
const Input = ({props }) => { 
    const [inputValue, setValue] = useState(tree);
 // let prepend = (inputValue) =>{
   // let res= `JSON.parse(\`${inputValue}\`);\n let binaryTree = ${binaryTree} \n binaryTree(tree)`
  //  return res
  //  }
  
    let toObjectLiteral= (inputValue)=>{
      let unquoted = inputValue.replace(/"([^"]+)":/g, '$1:');
      return unquoted

    }
    function preorder(tree, res) {
      if (!tree) return res;
     res.push(tree.val);
      preorder(tree.left, res);
      preorder(tree.right, res);
      return res;
    }
    let parsedTree= JSON.parse(inputValue);
    const PrettyPrintJson = ({data}) => (<div><pre class='input__json'>{JSON.stringify(data, undefined, 10) }</pre></div>);
      return(
        <>
      <div className="input">
     
          <h1 className="input__intro">Binary Tree Visualization </h1>
          <div className="input__box">
            <textarea
              className="input__textbox"
              type="text"
              id="input"
              value={inputValue}
          
              onChange={(e) => setValue(e.target.value)}
              rows={0}
              cols={0}
            >hi</textarea>
              <p className="input__input-value">toObjectLiteral:
              <span className="input__highlight">{toObjectLiteral(inputValue)}</span></p>
              <p className="input__array-value">toPreorderArray:
              <span className="input__array-highlight"> [{preorder(parsedTree, [])}]</span></p>
              <PrettyPrintJson data={ jtree } />
          </div>
     
  
        </div>
      </>    
      );
    
  }
  export default Input;


  // <Simulator inputtree={prepend(inputValue)}/>
















  //  <label className="label">Enter Binary Input</label>
  //    <p className="input-value">The value of the input is: <span className="highlight">{this.state.value}</span></p>
/*    const handleChange = e => {
      this.setState({value: e.target.value});
    };*/

    //   const [inputValue] = useState([{ text: 'Learn Hooks' }]);[inputValue]= useState(inputValue) {inputValue.map(inputValue => <div>{inputValue.text}</div>)}
       //  return unquoted.replace(/,/g, "<br/>");
  // return unquoted.replace(/,/g, '\n');
   //  let newLine =unquoted.split(',');
   // return unquoted.split(',');