
import React, { Component } from "react";
import './Input.scss'
//import axios from 'axios';


import '../Gallery/Gallery';

class Input extends Component {
    constructor(props) {
      super(props);
      this.state = {value: '{val:{}}'};
      this.handleChange = this.handleChange.bind(this);
    }
    
    handleChange(e) {
      this.setState({value: e.target.value});
    }
  
    
    render() {
      return(
        <div className="container">
          <h1 className="intro">Binary Tree Visualization</h1>
          <div className="box">
            <label className="labela">Enter Binary Input</label>
   <textarea className="input-is-medium" type='text' id='input' value={this.state.value} onChange={this.handleChange} 
             rows={5}
            cols={5}
  />
            <p className="input-value">The value of the input is: <span className="highlight">{this.state.value}</span></p>
          </div>
        </div>
      );
    }
  }

  
  export default Input;
  