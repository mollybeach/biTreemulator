
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
    function preOrder(tree, res) {
      if (!tree) return res;
     res.push(tree.val);
      preOrder(tree.left, res);
      preOrder(tree.right, res);
      return res;
    }
    function inOrder(tree, res) {
      if (!tree) return res;
      inOrder(tree.left, res);
      res.push(tree.val);
      inOrder(tree.right, res);
      return res;
    }
    function postOrder(tree, res) {
      if (!tree) return res;
      postOrder(tree.left, res);
      postOrder(tree.right, res);
      res.push(tree.val);
      return res;
    }
    let parsedTree= JSON.parse(inputValue);
  
      return(
        <>
      <div className="input">   
          <h1 className="input__intro">Binary Tree Visualization </h1>
          <div className="input__box">
            <pre class='input__json'>{JSON.stringify(jtree, undefined, 10) }<textarea
              className="input__textbox"
              type="text"
              id="input"
              value=''
             
              onChange={(e) => setValue(e.target.value)}
              rows={0}
              cols={0}
            >hi</textarea></pre>
              <p className="input__input-value">toObjectLiteral:
              <div className="input__highlight">{toObjectLiteral(inputValue)}</div></p>
              <div className="input__inorder-value">InOrder: (Left, Root, Right) 
              <div className="input__inorder-highlight"> [{inOrder(parsedTree, [])}]</div></div>
              <div className="input__array-value">Preorder: (Root, Left, Right)
              <div className="input__array-highlight"> [{preOrder(parsedTree, [])}]</div></div>
              <div className="input__array-value">Postorder: (Left, Right, Root)
              <div className="input__array-highlight"> [{postOrder(parsedTree, [])}]</div></div>
           
          </div>
        </div>
      </>    
      );
    
  }
  export default Input;


  // <Simulator inputtree={prepend(inputValue)}/>




//   <PrettyPrintJson data={ jtree } />
//   const PrettyPrintJson = ({data}) => (<div><pre class='input__json'>{JSON.stringify(data, undefined, 10) }</pre></div>);











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