import React, { Component } from "react";
import axios from 'axios';
import chevronImg from "../../assets/Icons/chevron_right-24px.svg";

import Input from '../Input/Input.jsx'
import './Gallery.scss';
import '../Input/Input.scss';

class Gallery extends Component {
  
      state = {
        treeList: [],
        draw: ''
      }
    
      componentDidMount(){
        axios.get('http://localhost:8080/gallery')
        .then(res=>{
          this.setState({
            treeList:res.data
          })
          })
          axios.get('http://localhost:8080/gallery')
          .then(res=>{
            this.setState({
              draw:res.data
            })
            })
          
      }
render(){
  //const { treeList} = this.state;
        return (
        <>
        <div className="gallery">
    
       
          <div className='gallery__inside '> 
          <div className="gallery__title "> biTreemulator:
            <img className="gallery__arrow" src={chevronImg} alt="img" />
          </div>
          <Input treeList = {this.state.treeList}  draw = {this.state.draw}/>
  
          </div>
        </div>
      

      </>   
        );
        }
}

export default Gallery;
//  <div className="gallery__subtitle "> Simulated Display: </div>
