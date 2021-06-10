

//import React, { useEffect, useState,useRef  } from "react";
import React, { useState } from "react";
import './Input.scss'
//import axios from 'axios';
import {tree} from '../Simulator/tree'

const Input = ({x }) => { 
    const [inputValue, setValue] = useState(tree);
  //  const [example1, example2] = useState(inputValue);
     
      return(
        <>
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
      </>
      
      );
    
  }

//   const [inputValue] = useState([{ text: 'Learn Hooks' }]);[inputValue]= useState(inputValue) {inputValue.map(inputValue => <div>{inputValue.text}</div>)}
  export default Input;
  //  <label className="label">Enter Binary Input</label>
  //    <p className="input-value">The value of the input is: <span className="highlight">{this.state.value}</span></p>
/*    const handleChange = e => {
      this.setState({value: e.target.value});
    };*/