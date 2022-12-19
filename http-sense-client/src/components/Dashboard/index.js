import React, { useContext, useState, useEffect } from 'react';
import { Flex, Heading, useDisclosure } from '@chakra-ui/react';
import SideDrawer from '../Drawer';

// components
import Chart from '../Chart';
import TableView from '../TableVeiw';

// state
import NetworkContext from '../../store/networkContext';

const Dashboard = () => {
  const { traffic, selectedRow } = useContext(NetworkContext);
  const { isOpen, onOpen, onClose } = useDisclosure();

  const [isMobile, setIsMobile] = useState(false);

  const handleResize = () => {
    if (window.innerWidth < 720) {
      setIsMobile(true);
    } else {
      setIsMobile(false);
    }
  };

  useEffect(() => {
    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  });

  return (
    <>
      <Flex w="85%" p="3%" flexDir="column" overflow="auto" minH="100vh">
        <Heading mb={4} fontWeight="normal">
          Network Activity
        </Heading>

        {!isMobile && (
          <>
            <Heading as="h2" fontWeight="normal" size="lg" mt="2rem" mb="2rem">
              Overview
            </Heading>
            <Flex w="40%">
              <Chart />
            </Flex>
          </>
        )}

        <Flex justifyContent="space-between" mt="5rem">
          <Flex>
            <Heading as="h2" size="lg" fontWeight="normal">
              Web Transactions
            </Heading>
          </Flex>
        </Flex>
        <Flex flexDir="column">
          <Flex overflow="auto">
            <TableView traffic={traffic} onOpen={onOpen} />
          </Flex>
        </Flex>
      </Flex>
      <SideDrawer onClose={onClose} isOpen={isOpen} selectedRow={selectedRow} />
    </>
  );
};

export default Dashboard;
