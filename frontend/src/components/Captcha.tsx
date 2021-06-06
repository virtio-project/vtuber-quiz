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
        size={props.size === undefined ? "invisible": props.size}
        onExpire={onExpire}
        onVerify={onVerify}
      />
      <input name="hCaptcha-token" hidden={true} value={token === null ? "" : token}/>
      <Label4 className={css({ color: '#555', 'padding-bottom': '7px' })}>
        This site is protected by hCaptcha and its <a
          className={css({ color: '#555', 'text-decoration': 'none' })}
          href="https://hcaptcha.com/privacy">Privacy Policy</a> and <a
          className={css({ color: '#555', 'text-decoration': 'none' })}
          href="https://hcaptcha.com/terms">Terms of Service</a> apply.
      </Label4>
    </React.Fragment>
  );
});
