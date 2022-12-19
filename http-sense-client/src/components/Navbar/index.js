import { Flex, Image, Icon, Link, Text } from '@chakra-ui/react';
import { FiActivity } from 'react-icons/fi';

const NavBar = () => {
  return (
    <Flex
      w="15%"
      flexDir="column"
      alignItems="center"
      backgroundColor="#171810"
      color="#fff"
    >
      <Flex flexDir="column" justifyContent="space-between" height="100vh">
        <Flex flexDir="column" as="nav" alignItems="center">
          <Image src="/logo.png" alt="http sense logo" maxW="100%" />
          <Flex
            flexDir="row"
            align="flex-start"
            justifyContent="center"
            className="main-nav-items"
          >
            {/* <Link>
              <Icon as={FiActivity} fontSize="2xl" className="active-icon" />
            </Link> */}
            {/* <Link _hover={{ textDecor: 'none' }}>
              <Text className="active">Monitor</Text>
            </Link> */}
          </Flex>
        </Flex>

        <Flex flexDir="column" alignItems="center" mb={10} mt={5}>
          <Text textAlign="center" fontSize="1.2rem">
            {' '}
            Made with 💚
          </Text>
        </Flex>
      </Flex>
    </Flex>
  );
};

export default NavBar;
