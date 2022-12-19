import {
  DrawerCloseButton,
  DrawerContent,
  DrawerHeader,
  DrawerOverlay,
  Heading,
  Drawer,
  DrawerBody,
  Icon,
  Text,
  Code,
  Flex,
} from '@chakra-ui/react';
import React from 'react';

import { getColorCode } from '../../utils';
import { FiArrowDown } from 'react-icons/fi';

const SideDrawer = ({ selectedRow, isOpen, onClose }) => {
  return (
    <Drawer
      isOpen={isOpen}
      placement="right"
      size="xl"
      onClose={onClose}
      portalProps={selectedRow}
    >
      <DrawerOverlay />
      <DrawerContent>
        <DrawerCloseButton size={'lg'} />
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

                <Code
                  wordBreak="break-word"
                  fontSize="1.4rem"
                  variant="subtle"
                  colorScheme="yellow"
                >
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
                      wordBreak="break-word"
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

                  <Code
                    wordBreak="break-word"
                    fontSize="1.4rem"
                    variant="subtle"
                    colorScheme="teal"
                  >
                    {Object.keys(selectedRow.response?.headers).map(key => {
                      return (
                        <React.Fragment key={key}>
                          {key}: {selectedRow.response?.headers[key]};
                          <br />
                        </React.Fragment>
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
                  <Code
                    fontSize="1.4rem"
                    variant="subtle"
                    colorScheme="teal"
                    wordBreak="break-word"
                  >
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
  );
};

export default SideDrawer;
