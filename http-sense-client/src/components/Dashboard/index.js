import {
  Flex,
  Heading,
  Icon,
  Link,
  Slide,
  Text,
  useDisclosure,
} from '@chakra-ui/react';
import { FiActivity } from 'react-icons/fi';

// components
import Chart from '../Chart';
import TableView from '../TableVeiw';

const Dashboard = () => {
  const { isOpen, onToggle } = useDisclosure();

  return (
    <Flex h="100vh" flexDir="row" overflow="hidden" maxW="200rem">
      {/* Column 1 */}
      <Flex
        w="15%"
        flexDir="column"
        alignItems="center"
        backgroundColor="#0b0b0b"
        color="#fff"
      >
        <Flex flexDir="column" justifyContent="space-between" height="100vh">
          <Flex flexDir="column" as="nav">
            <Heading
              mt={50}
              mb={100}
              fontSize="4xl"
              letterSpacing="wide"
              alignSelf="center"
            >
              HTTP SENSE
            </Heading>
            <Flex
              flexDir="row"
              align="flex-start"
              justifyContent="center"
              className="main-nav-items"
            >
              <Link>
                <Icon as={FiActivity} fontSize="2xl" className="active-icon" />
              </Link>
              <Link _hover={{ textDecor: 'none' }}>
                <Text className="active">Monitor</Text>
              </Link>
            </Flex>
          </Flex>

          <Flex flexDir="column" alignItems="center" mb={10} mt={5}>
            <Text textAlign="center" fontSize="1.2rem">
              {' '}
              Made with ❤️
            </Text>
          </Flex>
        </Flex>
      </Flex>
      {/* Column 2 */}
      <Flex w="60%" p="3%" flexDir="column" overflow="auto" minH="100vh">
        <Heading mb={4} fontWeight="normal">
          Network Activity
        </Heading>
        <Text color="#555" fontSize="1.8rem">
          Overview
        </Text>
        <Chart />
        <Flex justifyContent="space-between" mt={8}>
          <Flex>
            <Heading as="h2" size="lg" fontWeight="normal">
              Web Transactions
            </Heading>
          </Flex>
        </Flex>
        <Flex flexDir="column">
          <Flex overflow="auto">
            <TableView onToggle={onToggle} />
          </Flex>
        </Flex>
      </Flex>
      {/* Column 3 */}
      {/* <Slide direction="right" in={isOpen}> */}
      <Flex w="30%" p="3%" bgColor="#f5f6fa" flexDir="column" overflow="auto">
        <Heading mb={4} fontWeight="normal">
          Request Details
        </Heading>
      </Flex>
      {/* </Slide> */}
    </Flex>
  );
};

export default Dashboard;
