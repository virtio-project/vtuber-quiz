import React from 'react';
import {
  HeaderNavigation,
  ALIGN,
  StyledNavigationItem as NavigationItem,
  StyledNavigationList as NavigationList,
} from 'baseui/header-navigation';
import {StyledLink as Link} from 'baseui/link';
import {Button} from 'baseui/button';

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
          <Link href="#">Tab Link One</Link>
        </NavigationItem>
        <NavigationItem>
          <Link href="#">Tab Link Two</Link>
        </NavigationItem>
      </NavigationList>
      <NavigationList $align={ALIGN.right}>
        <NavigationItem>
          <Button>Get started</Button>
        </NavigationItem>
      </NavigationList>
    </HeaderNavigation>
  );
};

export default Nav;