import React from 'react';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link
} from "react-router-dom";
import Login from "./pages/Login";
import bg from './assests/jezael-melgoza.jpg';
import Nav from "./components/Nav";
import {useStyletron} from "baseui";

const App: React.FC = () => {
  const [css] = useStyletron();
  return (
    <Router>
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

        {/* A <Switch> looks through its children <Route>s and
            renders the first one that matches the current URL. */}
        <Switch>
          <Route path="/login">
            <Login />
          </Route>
          <Route path="/users">
            <Users />
          </Route>
          <Route path="/">
            <Home />
          </Route>
        </Switch>

        <footer className={css({
          height: '120px',
          backgroundColor: '#666'
        })}>

        </footer>


      </div>
    </Router>
  );
}

function Home() {
  return <h2>Home</h2>;
}

function About() {
  return <h2>About</h2>;
}

function Users() {
  return <h2>Users</h2>;
}

export default App;
