import { useContext } from 'react';
import {
  Flex,
  Heading,
  Text,
  Code,
  useDisclosure,
  DrawerOverlay,
  Drawer,
  DrawerContent,
  DrawerHeader,
  DrawerBody,
  Icon,
} from '@chakra-ui/react';

// components
import Chart from '../Chart';
import TableView from '../TableVeiw';

// state
import NetworkContext from '../../store/networkContext';

import { getColorCode } from '../../utils';
import { FiArrowDown } from 'react-icons/fi';

const Dashboard = () => {
  const { traffic, selectedRow } = useContext(NetworkContext);
  const { isOpen, onOpen, onClose } = useDisclosure();

  return (
    <>
      <Flex w="85%" p="3%" flexDir="column" overflow="auto" minH="100vh">
        <Heading mb={4} fontWeight="normal">
          Network Activity
        </Heading>
        <Heading as="h2" fontWeight="normal" size="lg" mt="2rem" mb="2rem">
          Overview
        </Heading>
        <Flex w="40%">
          <Chart />
        </Flex>

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

      <Drawer
        isOpen={isOpen}
        placement="right"
        size="lg"
        onClose={onClose}
        portalProps={selectedRow}
      >
        <DrawerOverlay />
        <DrawerContent>
          <DrawerHeader textAlign="center">
            <Heading fontWeight="normal" mt="2rem">
              Transcation Details
            </Heading>
          </DrawerHeader>
          {(selectedRow && (
            <DrawerBody>
              <Flex p="5%" gap="4rem" justifyContent="center" flexWrap="wrap">
                <Flex flexDir="column" alignItems="center">
                  <Text fontSize="1.4rem" fontWeight="semibold">
                    PATH
                  </Text>
                  <Text fontSize="1.4rem">
                    {selectedRow.request.method} {selectedRow.request.uri}
                  </Text>
                </Flex>
                <Flex flexDir="column" alignItems="center">
                  <Text fontSize="1.4rem" fontWeight="semibold">
                    STATUS
                  </Text>
                  <Text
                    fontSize="1.4rem"
                    color={getColorCode(selectedRow.response?.status_code)}
                  >
                    {(selectedRow.response?.status_code &&
                      selectedRow.response?.status_code) ||
                      '...'}
                  </Text>
                </Flex>
                <Flex flexDir="column" alignItems="center">
                  <Text fontSize="1.4rem" fontWeight="semibold">
                    TYPE
                  </Text>
                  <Flex>
                    <Text fontSize="small">Incoming </Text>
                    <Icon color="#2ed573" as={FiArrowDown} fontSize="2xl" />
                  </Flex>
                </Flex>
                <Flex flexDir="column" alignItems="center">
                  <Text fontSize="1.4rem" fontWeight="semibold">
                    SIZE
                  </Text>
                  <Text fontSize="1.4rem">
                    {(selectedRow.response?.body_size >= 0 &&
                      `${selectedRow.response?.body_size} B`) ||
                      '...'}
                  </Text>
                </Flex>
                <Flex flexDir="column" alignItems="center">
                  <Text fontSize="1.4rem" fontWeight="semibold">
                    TIME
                  </Text>
                  <Text fontSize="1.4rem">
                    {(selectedRow.response?.createdAt -
                      selectedRow.request.createdAt >
                      0 &&
                      `${
                        selectedRow.response?.createdAt -
                        selectedRow.request.createdAt
                      } ms`) ||
                      '...'}{' '}
                  </Text>
                </Flex>
                <Flex flexDir="column" alignItems="center">
                  <Text fontSize="1.4rem" fontWeight="semibold">
                    REQUEST HEADERS
                  </Text>
                  <Code fontSize="1.4rem" variant="subtle" colorScheme="yellow">
                    {Object.keys(selectedRow.request.headers).map(key => {
                      return (
                        <>
                          {key}: {selectedRow.request.headers[key]};
                          <br />
                        </>
                      );
                    })}
                  </Code>
                </Flex>
                {(selectedRow.request.body &&
                  'POST,PUT,PATCH'.includes(selectedRow.request.method) && (
                    <Flex flexDir="column" alignItems="center">
                      <Text fontSize="1.4rem" fontWeight="semibold">
                        REQUEST BODY
                      </Text>
                      <Code
                        fontSize="1.4rem"
                        variant="subtle"
                        colorScheme="yellow"
                      >
                        {selectedRow.response?.body}
                      </Code>
                    </Flex>
                  )) ||
                  null}
                {(selectedRow.response && (
                  <Flex flexDir="column" alignItems="center">
                    <Text fontSize="1.4rem" fontWeight="semibold">
                      RESPONSE HEADERS
                    </Text>
                    <Code fontSize="1.4rem" variant="subtle" colorScheme="teal">
                      {Object.keys(selectedRow.response?.headers).map(key => {
                        return (
                          <>
                            {key}: {selectedRow.response?.headers[key]};
                            <br />
                          </>
                        );
                      })}
                    </Code>
                  </Flex>
                )) ||
                  null}
                {(selectedRow.response && (
                  <Flex flexDir="column" alignItems="center">
                    <Text fontSize="1.4rem" fontWeight="semibold">
                      RESPONSE BODY
                    </Text>
                    <Code fontSize="1.4rem" variant="subtle" colorScheme="teal">
                      {selectedRow.response?.body}
                    </Code>
                  </Flex>
                )) ||
                  null}
              </Flex>
            </DrawerBody>
          )) ||
            null}
        </DrawerContent>
      </Drawer>
    </>
  );
};

export default Dashboard;
