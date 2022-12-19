import { createContext, useState, useContext, useEffect } from 'react';

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
  const [hasChannel, setHasChannel] = useState(false);
  const [isBootstrapped, setIsBootstrapped] = useState(false);

  const updateSelectedRow = row => {
    setSelectedRow(row);
  };

  const context = {
    traffic,
    selectedRow,
    updateSelectedRow,
  };

  // subscribe to channel for realtimed data
  useEffect(() => {
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
  }, [isLoggedIn, supabase]);

  if (!hasChannel) {
    setHasChannel(true);
  }

  async function bootstrap_requests() {
    console.log('Bootstrapping');
    // GUYS 🚨!! if you're seeing this, this is a huge bug (T_T)
    // If I remove these empty calls to the response table supabase returns no response
    // seems to be a bug in the supabase js lib, please have a look, feel free to reach out to us for any queries
    // @tchyut_p on Twitter
    await supabase.from('response').select();
    await supabase.from('response').select();
    const responses = await supabase.from('response').select();
    const requests = await supabase.from('request').select();

    setTraffic(prevTraffic => {
      let prevRequestIds = new Set(prevTraffic.map(x => x.id));

      let traffic = [
        ...prevTraffic,
        ...requests.data
          .filter(x => !prevRequestIds.has(x.id))
          .map(x => ({
            id: x.id,
            request: x.content,
          })),
      ];

      for (const response of responses.data) {
        for (let item of traffic) {
          if (item.id === response.request_id) {
            item.response = response.content;
          }
        }
      }

      setIsBootstrapped(true);

      return traffic;
    });
  }

  isLoggedIn && !isBootstrapped && bootstrap_requests();

  return (
    <NetworkContext.Provider value={context}>
      {props.children}
    </NetworkContext.Provider>
  );
};

export default NetworkContext;
