import React from 'react';
import { Link, NavLink } from 'react-router-dom';
import company from '../../assets/Icons/company.svg';
import './Header.scss';

function Header() {
  return (
    <header className="header">
      <nav className="header__nav">
      <Link to="/" className="header__logo" alt="Binary Treeulator Logo">   
          <img className ="header__rainbow"src={company} alt=''></img>
          </Link>
        <ul className="header__nav-list"> 
          <li>
            <NavLink to="/" exact className="header__nav-item" activeClassName="header__nav-item--active"><span>Home</span></NavLink>
          </li>
          <li>
            <NavLink to="/simulator" className="header__nav-item" activeClassName="header__nav-item"><span>Simulator</span></NavLink>
          </li>
          <li>
            <NavLink to="/uploadfile" className="header__nav-item" activeClassName="header__nav-item"><span>Upload</span></NavLink>
          </li>
         
        </ul>

   
      </nav>

    </header>
  )
}


export default Header;