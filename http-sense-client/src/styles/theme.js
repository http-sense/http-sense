import { extendTheme, theme as chakraTheme } from '@chakra-ui/react';

const fonts = {
  ...chakraTheme,
  heading: `'Space Mono', sans-serif`,
  body: `'Space Mono', sans-serif`,
};

const theme = extendTheme({
  fonts,
});
export default theme;
