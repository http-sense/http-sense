import { useContext, useEffect, useState } from 'react';

import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Filler,
  Legend,
} from 'chart.js';
import { Line } from 'react-chartjs-2';
import NetworkContext from '../../store/networkContext';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Filler,
  Legend
);

const options = {
  responsive: true,
  plugins: {
    legend: {
      position: 'top',
    },
    title: {
      display: false,
    },
  },
  maintainAspectRatio: true,
  scales: {
    x: {
      grid: {
        display: false,
      },
    },
    y: {
      grid: {
        borderDash: [3, 3],
      },
      // beginAtZero: true, // this works
    },
  },
};

const Chart = () => {
  const { traffic } = useContext(NetworkContext);

  const requestsData = traffic.map(t => {
    return {
      id: t.request.id,
      initiatedAt: t.request.createdAt,
      timeTaken: t.response?.createdAt - t.request.createdAt,
      path: t.request.uri,
      verb: t.request.method,
      status: t.response?.status_code,
    };
  });

  const labels = requestsData.map(t => {
    const date = new Date(t.initiatedAt);
    return `${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}`;
  });

  const datasets = [
    {
      label: 'Response Matrix',
      fill: true,
      lineTension: 0.5,
      backgroundColor: '#adefc8',
      borderColor: '#2ed573',
      borderCapStyle: 'butt',
      borderDashOffset: 0.0,
      borderJoinStyle: '#2ed573',
      pointBorderColor: '#2ed573',
      pointBackgroundColor: '#fff',
      pointBorderWidth: 1,
      pointHoverRadius: 5,
      pointHoverBackgroundColor: '#2ed573',
      pointHoverBorderColor: '#2ed573',
      pointHoverBorderWidth: 2,
      pointRadius: 1,
      pointHitRadius: 10,
      data: requestsData.map(t => t.timeTaken),
    },
  ];

  return <Line data={{ labels, datasets }} options={options} />;
};
export default Chart;
