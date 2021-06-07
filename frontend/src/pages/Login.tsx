import React from 'react';
import LoginForm from '../components/LoginForm';
import {Cell, Grid} from "baseui/layout-grid";
import {H2} from "baseui/typography";

const Login: React.FC = () => {
  return (
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
  );
}

export default Login;
