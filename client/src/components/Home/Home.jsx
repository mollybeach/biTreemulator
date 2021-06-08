import React, { Component } from "react";
import { Link } from 'react-router-dom';
import chevronImg from "../../assets/Icons/chevron_right-24px.svg";
import "./Home.scss";


class Home extends Component {
  render() {
    return (
      <>
        <div className="home">
          <div className="hero-container "></div>
          <div className="content"></div>
          <div className='home__inside '>
          <div className="home__title ">binary tree simulator
          <img className="home__arrow" src={chevronImg} alt="img" />
          </div>
          <div className="home__subtitle ">Welcome to biTreemulator!</div>
          <div className="home__about " >
            Start your journey by entering a binary tree object or array. 
            <div className="home__buttons">
            <Link to="/uploadfile">  <button className="btn btn--delta" type="submit"><span>Get Started</span></button></Link>
            </div>
          </div>
          </div>
        </div>
        <div className="content">
        </div>
        <div className="content"></div>
      </>
    );
  }
}

export default Home;
  
  