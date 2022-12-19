import { useEffect, useContext } from 'react';

// external
import { createClient } from '@supabase/supabase-js';

// components
import Dashboard from '../Dashboard';
import NavBar from '../Navbar';

// store
import AuthContext from '../../store/authContext';

// styles
import theme from '../../styles/theme';
import { ChakraProvider, Flex } from '@chakra-ui/react';
import { NetworkContextProvider } from '../../store/networkContext';
import Home from '../Home';

const supabase = createClient(
  process.env.REACT_APP_SUPABASE_URL,
  process.env.REACT_APP_SUPABASE_KEY
);

const url = new URL(window.location.href);

const [email, password] = (url.hash &&
  window.atob(url.hash.slice(1)).split('::')) || [null, null];

const credentials = {
  email,
  password,
};

function App() {
  const { handleLogin, updateSupabaseInstance } = useContext(AuthContext);

  useEffect(() => {
    (async () => {
      const { data, error } = await supabase.auth.signInWithPassword(
        credentials
      );

      if (error) {
        console.error(error);
      }

      handleLogin();
      updateSupabaseInstance(supabase);
    })();
  }, [handleLogin, updateSupabaseInstance]);

  return (
    <ChakraProvider theme={theme}>
      <NetworkContextProvider>
        <Flex h="100vh" flexDir="row" overflow="hidden" maxW="200rem">
          <NavBar />
          {(url.hash && <Dashboard />) || <Home />}
        </Flex>
      </NetworkContextProvider>
    </ChakraProvider>
  );
}

export default App;
