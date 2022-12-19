import { useContext } from 'react';

import {
  Flex,
  Icon,
  Table,
  Tbody,
  Td,
  Text,
  Th,
  Thead,
  Tr,
} from '@chakra-ui/react';
import { FiArrowDown } from 'react-icons/fi';

// state
import NetworkContext from '../../store/networkContext';

import { getColorCode } from '../../utils';

const handleRowClick = (onOpen, updateSelectedRow, rowTraffic) => {
  onOpen();
  updateSelectedRow(rowTraffic);
};

const renderRows = (traffic, onOpen, updateSelectedRow) => {
  return (
    (traffic.length &&
      traffic.map(t => {
        const responseTime = t.response?.createdAt - t.request.createdAt;
        const statusCode = t.response?.status_code;
        const contentLegth = t.response?.body_size;

        return (
          <Tr
            key={t.request.createdAt}
            onClick={() => handleRowClick(onOpen, updateSelectedRow, t)}
            className="table-row"
          >
            <Td>
              <Text fontSize="small" size="sm" fontWeight="semibold">
                {t.request.uri}
              </Text>
            </Td>
            <Td>
              <Text fontSize="small">{t.request.method}</Text>
            </Td>
            <Td>
              <Text
                color={getColorCode(statusCode)}
                fontSize="small"
                fontWeight="semibold"
              >
                {(statusCode && statusCode) || '...'}
              </Text>
            </Td>
            <Td>
              <Flex gap={3}>
                <Text fontSize="small">Incoming </Text>
                <Icon color="#2ed573" as={FiArrowDown} fontSize="2xl" />
              </Flex>
            </Td>
            <Td>
              <Text fontSize="small">
                {(contentLegth >= 0 && `${contentLegth} B`) || '...'}
              </Text>
            </Td>
            <Td>
              <Text fontSize="small">
                {(responseTime > 0 && `${responseTime} ms`) || '...'}{' '}
              </Text>
            </Td>
          </Tr>
        );
      })) ||
    null
  );
};

const TableView = ({ traffic, onOpen }) => {
  const { updateSelectedRow } = useContext(NetworkContext);
  return (
    <Table variant="simple" marginTop="2rem">
      <Thead>
        <Tr color="#555">
          <Th>
            <Text fontSize="small">Path</Text>
          </Th>
          <Th>
            <Text fontSize="small">Verb</Text>
          </Th>
          <Th>
            <Text fontSize="small">Status</Text>
          </Th>
          <Th>
            <Text fontSize="small">Type</Text>
          </Th>
          <Th>
            <Text fontSize="small">Size</Text>
          </Th>
          <Th>
            <Text fontSize="small">Time</Text>
          </Th>
        </Tr>
      </Thead>
      <Tbody>{renderRows(traffic, onOpen, updateSelectedRow)}</Tbody>
    </Table>
  );
};

export default TableView;
