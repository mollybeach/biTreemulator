

//import React, { useEffect, useState,useRef  } from "react";
import React, { useState } from "react";
import './Input.scss'
//import axios from 'axios';

const Input = ({ simulatorList }) => { 
    const [inputValue, setValue] = useState("hellaw");
    const [example1, example2] = useState(inputValue);

    const handleChange = e => {
      this.setState({value: e.target.value});
    };
     
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
/*
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
  }*/