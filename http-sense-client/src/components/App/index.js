import { ChakraProvider } from '@chakra-ui/react';
import Dashboard from '../Dashboard';

// styles
import theme from '../../styles/theme';

function App() {
  return (
    <ChakraProvider theme={theme}>
      <Dashboard />
    </ChakraProvider>
  );
}

export default App;
