import React from 'react';
import {
  HeaderNavigation,
  ALIGN,
  StyledNavigationItem as NavigationItem,
  StyledNavigationList as NavigationList,
} from 'baseui/header-navigation';
import { StyledLink } from 'baseui/link';
import { Button } from 'baseui/button';
import { Link as RouterLink } from 'react-router-dom';
import {useStyletron} from "baseui";

type LinkProps = {
  to: string;
  children?: React.ReactNode;
};

const Link: React.FC<LinkProps> = ({to, children}) => {
  const [css] = useStyletron();
  return (
    <RouterLink className={css({
      color: 'inherit',
      textDecoration: 'none'
    })} to={to}>{children}</RouterLink>
  )
}

const Nav: React.FC = () => {
  return (
    <HeaderNavigation overrides={{
      Root: {
        style: ({ _ }) => ({
          paddingLeft: '25px',
          paddingRight: '30px',
        })
      }
    }}>
      <NavigationList $align={ALIGN.left}>
        <NavigationItem>Vtuber 测试平台</NavigationItem>
      </NavigationList>
      <NavigationList $align={ALIGN.center} />
      <NavigationList $align={ALIGN.right}>
        <NavigationItem>
          <StyledLink href="#">Tab Link One</StyledLink>
        </NavigationItem>
        <NavigationItem>
          <StyledLink href="#">Tab Link Two</StyledLink>
        </NavigationItem>
      </NavigationList>
      <NavigationList $align={ALIGN.right}>
        <NavigationItem>
          <Button><Link to="/login">Login</Link></Button>
        </NavigationItem>
      </NavigationList>
    </HeaderNavigation>
  );
};

export default Nav;