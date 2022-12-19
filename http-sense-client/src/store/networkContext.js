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

    async function bootstrap_requests() {
      console.log("Bootstrapping");
      let requests = await supabase.from('request').select();
      let responses = await supabase.from('response').select();
          setTraffic(prevTraffic => {
            let prevRequestIds = new Set(prevTraffic.map(x => x.id));

            let traffic =  [...prevTraffic, ...requests.data.filter(x => !prevRequestIds.has(x.id)).map(x => ({
              id: x.id,
              request: x.content
            }))];

            for (const response of responses.data) {
              for (let item of traffic) {
                if (item.id === response.request_id) {
                  item.response = response.content;
                }
              }
            }
            console.log(traffic)
            return traffic;
          });
    }

    isLoggedIn && bootstrap_requests();


  return (
    <NetworkContext.Provider value={context}>
      {props.children}
    </NetworkContext.Provider>
  );
};

export default NetworkContext;
