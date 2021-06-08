import React, { Component } from "react";
import axios from 'axios';
import chevronImg from "../../assets/Icons/chevron_right-24px.svg";
import GS from './spCode.js';
import Input from '../Input/Input.jsx'
import './Gallery.scss';
import '../Input/input.scss';

class Gallery extends Component {
  
      state = {
        galleryList: [],
        draw: ''
      }
    
      componentDidMount(){
        axios.get('http://localhost:8080/gallery')
        .then(res=>{
          this.setState({
            galleryList:res.data
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
  //const { galleryList} = this.state;
        return (
        <>
        <div className="gallery">
            <div className="hero-container "></div>
            <div className="content"></div>
          <div className='gallery__inside '> 
          <div className="gallery__title "> biTreemulator:
            <img className="gallery__arrow" src={chevronImg} alt="img" />
          </div>
          <div className="gallery__subtitle "> Create 
          </div>
          <div className="gallery__subtitle "> Simulated Display: </div>
          <div className="gallery__about " >
          </div>
          </div>
        </div>
        <Input/>
        <GS galleryList = {this.state.galleryList}  draw = {this.state.draw}  />
      </>   
        );
        }
}

export default Gallery;

