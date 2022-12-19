import { createContext, useState } from 'react';

const AuthContext = createContext({
  supabase: null,
  isLoggedIn: false,
  handleLogin: () => {},
  updateSupabaseInstance: () => {},
});

export const AuthContextProvider = props => {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [supabaseInstance, setSupabaseInstance] = useState(null);

  const handleLogin = () => {
    setIsLoggedIn(true);
  };

  const updateSupabaseInstance = supabase => {
    setSupabaseInstance(supabase);
  };

  return (
    <AuthContext.Provider
      value={{
        isLoggedIn,
        supabase: supabaseInstance,
        handleLogin,
        updateSupabaseInstance,
      }}
    >
      {props.children}
    </AuthContext.Provider>
  );
};

export default AuthContext;
