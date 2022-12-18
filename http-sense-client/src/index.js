import { ColorModeScript } from '@chakra-ui/react';
import React from 'react';
import * as ReactDOM from 'react-dom/client';

// Components
import App from './components/App';

// Styles
import '@fontsource/montserrat';
import './styles/globals.css';

const container = document.getElementById('root');
const root = ReactDOM.createRoot(container);

root.render(
  <>
    <ColorModeScript />
    <App />
  </>
);
