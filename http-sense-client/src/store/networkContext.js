import { createContext, useState, useContext } from 'react';

import AuthContext from './authContext';

const NetworkContext = createContext({
  traffic: [],
  updateSelectedRow: () => {},
  selectedRow: null,
});

export const NetworkContextProvider = props => {
  const { isLoggedIn, supabase } = useContext(AuthContext);

  const [traffic, setTraffic] = useState([]);
  const [selectedRow, setSelectedRow] = useState(null);

  const updateSelectedRow = row => {
    setSelectedRow(row);
  };

  const context = {
    traffic,
    selectedRow,
    updateSelectedRow,
  };

  isLoggedIn &&
    supabase
      .channel('request-db-changes')
      .on(
        'postgres_changes',
        {
          event: 'INSERT',
          schema: 'public',
          table: 'request',
        },
        payload => {
          const request = payload.new.content;
          setTraffic(prevTraffic => {
            return [...prevTraffic, { id: payload.new.id, request }];
          });
        }
      )
      .on(
        'postgres_changes',
        {
          event: 'INSERT',
          schema: 'public',
          table: 'response',
        },
        payload => {
          setTraffic(prevTraffic => {
            const traffic = [...prevTraffic];
            for (let item of traffic) {
              if (item.id === payload.new.request_id) {
                item.response = payload.new.content;
              }
            }

            return traffic;
          });
        }
      )
      .subscribe();

  return (
    <NetworkContext.Provider value={context}>
      {props.children}
    </NetworkContext.Provider>
  );
};

export default NetworkContext;
