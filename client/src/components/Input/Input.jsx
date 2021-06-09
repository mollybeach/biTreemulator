
import React, { Component } from "react";
import './Input.scss'
//import axios from 'axios';

import GS from '../Gallery/spCode.js';

import '../Gallery/Gallery';
export let tree = {
  "val": 0,
  	"left": 	{
    	"val": 1,
    	     "left": { "val": 2, 
                		  "left": { "val": 3, 
                		             "left": { "val": 4, 
                                            "left":{ "val": 5, "left":null, "right":null},
                                            "right": { "val": 6, 
                                                        "left":{ "val": 7, "left":null, "right":null},
                                                        "right":{ "val": 8, "left":null, "right":null}
                                                      } 
                		                    }          
                            },
                		 "right": null
          			 },
          "right": { "val": 9, 
                  	  "left": { "val": 10, "left":null, "right":null},
                      "right": null
          		} 
  		    },
  
 	"right": {
    	 "val": 11,
           	"left": { "val": 12, 
                      "left": null, 
                      "right": null
                 },
   		  	 "right": {
     		      	"val": 13,
      			          "left": null,
     		             "right": { "val": 14, 
                                 "left": { "val": 15, 
                                            "left":{ "val": 16, 
                                                      "left":null, 
                                                      "right": { "val": 17, "left":null, "right":null}
                                                  }, 
                                            "right":null 
                		                    }, 
                                 "right": { "val": 18, 
                                            "left":null, 
                                            "right": { "val": 19, 
                                                        "left":{ "val": 20, "left":null, "right":null},
                                                        "right":null
                                                      }
                                        }
             }
           }
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
        
          </div>
          
        </div>
     
       <GS treeList = {this.state.value}  draw = {this.state.draw}  />
        </>
      );
    }
  }


  export default Input;
  //  <label className="label">Enter Binary Input</label>
  //    <p className="input-value">The value of the input is: <span className="highlight">{this.state.value}</span></p>