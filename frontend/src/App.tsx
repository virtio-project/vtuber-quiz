import React from 'react';
import './App.css';
import LoginForm from './components/LoginForm';
import Nav from './components/Nav';
import { FlexGrid, FlexGridItem } from 'baseui/flex-grid';
import { useStyletron } from 'baseui';


function App() {
  const [css] = useStyletron();
  return (
      <FlexGrid
        justifyContent='space-between'
        alignItems='flex-start'
        minHeight='100vh'
      >
        <FlexGridItem as='header'><Nav/></FlexGridItem>
        <FlexGridItem>
          <FlexGrid
            as='main'
            marginTop='scale800'
            paddingLeft='scale800'
            paddingRight='scale800'
            flexWrap={false}
            alignItems='center'
          >
            <FlexGridItem/>
            <hr className={css({
              height: '500px',
              margin: '0 20px',
              border: 'none',
              width: '2px',
              background: 'linear-gradient(rgba(255, 255, 255, 0), #555, #444, #555, rgba(255, 255, 255, 0))'
            })}/>
            <FlexGridItem
              paddingRight={['0', '0', 'scale2400', 'scale4800']}
              minWidth='300px'
              maxWidth='300px'
            >
              <LoginForm/>
            </FlexGridItem>
          </FlexGrid>
        </FlexGridItem>
        <FlexGridItem as='footer'></FlexGridItem>
      </FlexGrid>
  );
}

export default App;
