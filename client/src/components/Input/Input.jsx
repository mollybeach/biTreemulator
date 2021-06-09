
import React, { Component } from "react";
import './Input.scss'
//import axios from 'axios';
import GS from '../Gallery/spCode.js';

import '../Gallery/Gallery';
let tree = {
  val: 1,      
  left: { val: 2,            
                  left: { val: 3,        
                            left: { val: 4, left: null, right: null },
                            right: { val: 4, left: null, right: null }
                         },       
                     
                  right: { val: 3, left: null, right: null },
         },   
  right:  { val: 2, 
                  left: null,  
                  right: null,  
         }
}
;
class Input extends Component {
    constructor(props) {
      super(props);
      this.state = {value: JSON.stringify(tree)};
      this.handleChange = this.handleChange.bind(this);
    }
    
    handleChange(e) {
      this.setState({value: e.target.value});
    }
  
    
    render() {
      return(
        <>
        <div className="container">
          <h1 className="intro">Binary Tree Visualization </h1>
          <div className="box">
          
   <textarea className="input-is-medium" type='text' id='input' value={this.state.value} onChange={this.handleChange} 
             rows={5}
            cols={5}
  />
            <p className="input-value">The value of the input is: <span className="highlight">{this.state.value}</span></p>
          </div>
          
        </div>
       <GS treeList = {this.state.treeList}  draw = {this.state.draw}  />
        </>
      );
    }
  }

  
  export default Input;
  //  <label className="label">Enter Binary Input</label>