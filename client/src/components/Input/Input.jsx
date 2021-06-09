

//import React, { useEffect, useState,useRef  } from "react";
import React, { useState } from "react";
import './Input.scss'
//import axios from 'axios';

const Input = ({ simulatorList }) => { 
    const [inputValue, setValue] = useState("hellaw");
    const [subreddit, setSubreddit] = useState(inputValue);

    const handleChange = e => {
      this.setState({value: e.target.value});
    };
    
   
      return(
        <>
        <div className="container">
          <h1 className="intro">Binary Tree Visualization </h1>
          <div className="box">
            <textarea
              className="input-is-medium"
              type="text"
              id="input"
              value={inputValue}
              onChange={(e) => setValue(e.target.value)}
              rows={5}
              cols={5}
            />
              <p className="input-value">The value of the input is: <span className="highlight">{inputValue}</span></p>
          </div>
        </div>
      </>
      
      );
    
  }


  export default Input;
  //  <label className="label">Enter Binary Input</label>
  //    <p className="input-value">The value of the input is: <span className="highlight">{this.state.value}</span></p>