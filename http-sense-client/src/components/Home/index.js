import {
  Box,
  Button,
  Flex,
  Heading,
  HStack,
  Link,
  Text,
  VStack,
} from '@chakra-ui/react';
import React from 'react';
import { BiChevronRight } from 'react-icons/bi';

const Home = () => {
  return (
    <Box bg="#fff" width="100%">
      <VStack
        color="#171810"
        minH="100%"
        textAlign="center"
        align="center"
        justifyContent="center"
        spacing={6}
      >
        <Heading
          as="h1"
          fontSize="7xl"
          maxW={{ base: 'lg', md: 'xl', lg: '2xl' }}
        >
          Framework agnostic{' '}
          <Box
            as="span"
            bgGradient="linear-gradient(135deg, #38a2c0, #00bccf, #00d5c6, #00eba5, #09fd6e)"
            bgClip="text"
          >
            DevTools
          </Box>{' '}
          for the server
        </Heading>
        <Text color="blackAlpha.600" fontSize="3xl">
          Easy network monitoring for any microservice
        </Text>
        <HStack>
          <Link
            color="#22c35e"
            fontSize="4xl"
            fontWeight="bold"
            href="https://github.com/http-sense/http-sense#http-sense"
            target="_blank"
            textDecor="underline"
          >
            Get Started
          </Link>
        </HStack>
      </VStack>
    </Box>
  );
};

export default Home;