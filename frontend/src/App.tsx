import React from 'react';
import './App.css';
import LoginForm from './components/LoginForm';
import Nav from './components/Nav';
import { useStyletron } from 'baseui';
import bg from './assests/jezael-melgoza.jpg';
import {Cell, Grid} from "baseui/layout-grid";
import {H2} from "baseui/typography";


function App() {
  const [css] = useStyletron();
  return (
    <div
      className={css({
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'flex-start',
        height: '100vh',
        backgroundImage: `url(${bg})`
      })}
    >
      <header className={css({
          width: '100%',
          backgroundColor: '#fff'
      })}>
          <Nav/>
      </header>
      <Grid
        overrides={{
          Grid: {
            style: (_) => ({
              width: '100%',
              display: 'flex',
              flexGrow: 1,
              alignItems: 'center'
            })
          }
        }}
      >
        <Cell
          span={[0, 4, 8]}
          overrides={{
            Cell: {
              style: (_) => ({
                textAlign: 'center'
              })
            }
          }}
        >
          <H2>Welcome back!</H2>
        </Cell>
        <Cell span={4}>
          <LoginForm/>
        </Cell>
      </Grid>
      <footer className={css({
        height: '120px',
        backgroundColor: '#666'
      })}>

      </footer>
    </div>

  );
}

export default App;
