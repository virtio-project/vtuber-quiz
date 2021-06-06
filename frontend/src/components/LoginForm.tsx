import React from 'react';
import { FormControl } from 'baseui/form-control';
import { Input } from 'baseui/input';
import { Button } from 'baseui/button';
import { Captcha } from './Captcha';
import HCaptcha from '@hcaptcha/react-hcaptcha';

const LoginForm: React.FC = () => {
  const [username, setUsername] = React.useState<string | null>(null);
  const [password, setPassword] = React.useState<string | null>(null);
  const [token, setToken] = React.useState<string | null>(null);
  const [usernameErr, setUsernameErr] = React.useState(false);
  const [passwordErr, setPasswordErr] = React.useState(false);
  const captchaRef = React.useRef<HCaptcha>(null);

  const onSubmit = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    const invalidUsername = username === null || username.trim() === '';
    const invalidPassword = password === null || password.trim() === '';
    const formReady = !invalidUsername && !invalidPassword;

    setUsernameErr(invalidUsername);
    setPasswordErr(invalidPassword);
    if (formReady && token === null) {
      captchaRef.current?.execute()
    }

    console.log(`username: ${username}; password: ${password}; token: ${token}`)
  };

  return (
    <form>
      <FormControl>
        <Input
          placeholder="用户名"
          value={username === null ? '' : username}
          onChange={event => {
            setUsernameErr(false);
            setUsername(event.currentTarget.value);
          }}
          error={usernameErr}
          required
        />
      </FormControl>
      <FormControl>
        <Input
          placeholder="密码"
          type="password"
          value={password === null ? '' : password}
          onChange={event => {
            setPasswordErr(false);
            setPassword(event.currentTarget.value);
          }}
          error={passwordErr}
          required
        />
      </FormControl>
      <Captcha ref={captchaRef} onChange={setToken}/>
      <Button onClick={onSubmit}>登录</Button>
    </form>
  );
}

export default LoginForm;