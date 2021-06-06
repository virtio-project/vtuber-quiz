import React from 'react';
import HCaptcha from '@hcaptcha/react-hcaptcha';
import { Label4 } from 'baseui/typography';
import { useStyletron } from 'baseui';

type CaptchaProps = {
  size?: "normal" | "compact" | "invisible";
  onChange: (token: string | null) => any,
}

export const Captcha = React.forwardRef<HCaptcha, CaptchaProps>((props, ref) => {
  const [token, setToken] = React.useState<string | null>(null);
  const [css] = useStyletron();

  const onExpire = () => {
    setToken(null);
    props.onChange(token);
  }

  const onVerify = (newToken: string) => {
    setToken(newToken);
    props.onChange(newToken);
  }

  return (
    <React.Fragment>
      <HCaptcha
        ref={ref}
        sitekey="7a49b8da-6dab-4a83-9cbd-be7def92c75d"
        size={props.size === undefined ? "normal": props.size}
        onExpire={onExpire}
        onVerify={onVerify}
      />
      <input name="hCaptcha-token" hidden={true} value={token === null ? "" : token} onChange={(_) => {}}/>
      <Label4 className={css({ color: '#555', 'padding-bottom': '7px' })}>
        此网站受 hCaptcha® 保护，使用本网站表示您同意其
        <a className={css({ color: '#555', 'text-decoration': 'underline' })} href="https://hcaptcha.com/privacy">隐私政策</a>和
        <a className={css({ color: '#555', 'text-decoration': 'underline' })} href="https://hcaptcha.com/terms">用户协议</a>。
      </Label4>
    </React.Fragment>
  );
});
