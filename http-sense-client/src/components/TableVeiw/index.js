import {
  Heading,
  Table,
  Tbody,
  Td,
  Text,
  Th,
  Thead,
  Tr,
} from '@chakra-ui/react';

const TableView = ({ onToggle }) => {
  return (
    <Table variant="unstyled" marginTop="2rem">
      <Thead>
        <Tr color="#555">
          <Th>
            <Text fontSize="small">Request</Text>
          </Th>
          <Th>
            <Text fontSize="small">Verb</Text>
          </Th>
          <Th isNumeric>
            <Text fontSize="small">Status</Text>
          </Th>
          <Th>
            <Text fontSize="small">Type</Text>
          </Th>
          <Th>
            <Text fontSize="small">Size</Text>
          </Th>
          <Th isNumeric>
            <Text fontSize="small">Time</Text>
          </Th>
        </Tr>
      </Thead>
      <Tbody>
        <Tr onClick={onToggle}>
          <Td>
            <Heading fontSize="small" size="sm" fontWeight="semibold">
              GET https://google.com/hello
            </Heading>
          </Td>
          <Td>
            <Text fontSize="small">GET</Text>
          </Td>
          <Td>
            <Text color="#2ed573" fontSize="small" fontWeight="semibold">
              200
            </Text>
          </Td>
          <Td>
            <Text fontSize="small">xhr</Text>
          </Td>
          <Td>
            <Text fontSize="small">300B</Text>
          </Td>
          <Td>
            <Text fontSize="small">100ms</Text>
          </Td>
        </Tr>
        <Tr>
          <Td>
            <Heading fontSize="small" size="sm" fontWeight="semibold">
              https://twitter.com/profile
            </Heading>
          </Td>
          <Td>
            <Text fontSize="small">GET</Text>
          </Td>
          <Td>
            <Text color="#ff4757" fontSize="small" fontWeight="semibold">
              400
            </Text>
          </Td>
          <Td>
            <Text fontSize="small">xhr</Text>
          </Td>
          <Td>
            <Text fontSize="small">1KB</Text>
          </Td>
          <Td>
            <Text fontSize="small">100ms</Text>
          </Td>
        </Tr>
      </Tbody>
    </Table>
  );
};

export default TableView;
