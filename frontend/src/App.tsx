import React from 'react';
import './App.css';
import LoginForm from './components/LoginForm';
import Nav from './components/Nav';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <Nav/>
      </header>
      <LoginForm/>
    </div>
  );
}

export default App;
