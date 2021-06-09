import React, { Component } from 'react';
import {BrowserRouter, Route, Switch} from 'react-router-dom';
import axios from 'axios';
import Header from "./components/Header/Header";
import Home from './components/Home/Home';
import Simulator from './components/Simulator/Simulator';
import Input from './components/Input/Input';
import UploadFile from "./components/UploadFile/UploadFile";

import './App.scss'
import company from './assets/Icons/company.svg';
import ring from './assets/Icons/DNA-Circle.svg';


class App extends Component {
  state = {
    homeList : null,
    treeList : null,
    updateList: null,

}
componentDidMount(){
  axios.get('http://localhost:8080/home')
  .then(res=>{
    this.setState({
      homeList:res.data
    })
    axios.get('http://localhost:8080/simulator').then(res=>{
        this.setState({
          treeList:res.data
        })
  })

axios.get('http://localhost:8080/uploadfile').then(res=>{
  this.setState({
    uploadFileList:res.data
  })
})
    })
  
}

  render() {
    const { homeList ,treeList, uploadFileList} = this.state;
    if(homeList ===null ) {  
      return <> 
     <img className ="app__rainbows"src={company} alt=''></img>
       <div className = "app__load"></div>
       <div className = "app__text"> Thank you for waiting patiently. There's alot to load here! :) </div>
       <img className="app__ring" src={ring} alt=''></img>
       </>
       }
    return (
      <div className = 'app'>
        <BrowserRouter>
        <Header />
        <Switch>
          <Route exact path={[`/`, `/home`]} render = {(props)=> <Home   homeList = {homeList}   {...props}  />} />
          <Route exact path={[`/simulator`]} render = {(props)=> <Simulator treeList = {treeList}  {...props} />} />
          <Route exact path = '/uploadfile' render = {(props)=> <UploadFile uploadFileList = {uploadFileList}  {...props} />}  />
          <Route exact path = '/input' render = {(props)=> <Input treeList = {treeList}  {...props} />}  />
     
          </Switch> 
        </BrowserRouter>
      </div>
    );
  }
}

export default App;


