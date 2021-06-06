import React from 'react';
import { FormControl } from 'baseui/form-control';
import { Input } from 'baseui/input';
import { Button } from 'baseui/button';
import { useStyletron } from 'baseui';
import { Captcha } from './Captcha';
import HCaptcha from '@hcaptcha/react-hcaptcha';

const LoginForm: React.FC = () => {
  const [username, setUsername] = React.useState<string | null>(null);
  const [password, setPassword] = React.useState<string | null>(null);
  const [token, setToken] = React.useState<string | null>(null);
  const captchaRef = React.useRef<HCaptcha>(null);
  const [css] = useStyletron();

  const onSubmit = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    if (token === null) {
      captchaRef.current?.execute()
    }
    console.log(`username: ${username}; password: ${password}; token: ${token}`)
  };

  return (
    <form className={css({
      maxWidth: '300px',
      padding: '0 25px'
    })}>
      <FormControl>
        <Input
          placeholder="用户名"
          value={username === null ? '' : username}
          onChange={event => setUsername(event.currentTarget.value)}
          required
        />
      </FormControl>
      <FormControl>
        <Input
          placeholder="密码"
          type="password"
          value={password === null ? '' : password}
          onChange={event => setPassword(event.currentTarget.value)}
          required
        />
      </FormControl>
      <Captcha ref={captchaRef} onChange={setToken}/>
      <Button onClick={onSubmit}>登录</Button>
    </form>
  );
}

export default LoginForm;