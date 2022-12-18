import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

const data = {
  labels: [
    '0ms',
    '100ms',
    '200ms',
    '300ms',
    '400ms',
    '500ms',
    '600ms',
    '700ms',
    '800ms',
    '900ms',
    '1000ms',
  ],
  datasets: [
    {
      label: 'My Balance',
      fill: false,
      lineTension: 0.5,
      backgroundColor: '#db86b2',
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
      data: [500, 300, 400, 500, 800, 650, 700, 690, 1000, 1200, 1050, 1300],
    },
  ],
};

const options = {
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
  plugins: {
    legend: {
      display: false,
    },
  },
};

const Chart = () => <Line data={data} options={options} />;
export default Chart;
