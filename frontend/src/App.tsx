import React from 'react';
import './App.css';
import LoginForm from './components/LoginForm';
import Nav from './components/Nav';
import { useStyletron } from 'baseui';
import bg from './assests/jezael-melgoza.jpg';


function App() {
  const [css] = useStyletron();
  return (
    <div
      className={css({
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'flex-start',
        height: '100vh',
      })}
    >
      <header className={css({ width: '100%' })}><Nav/></header>
      <main
        className={css({
          display: 'flex',
          width: '100%',
          marginTop: 'scale800',
          flexGrow: 1,
          flexWrap: 'nowrap',
          alignItems: 'center',
          justifyContent: 'flex-end',
          backgroundImage: `url(${bg})`,
        })}
      >
        <section/>
        <section
          className={css({
            marginRight: '100px',
            width: '300px',
            height: '350px',
            contain: 'strict',
          })}
        >
          <LoginForm/>
        </section>
      </main>
      <footer className={css({
        height: '150px'
      })}>

      </footer>
    </div>

  );
}

export default App;
