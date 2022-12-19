import * as ReactDOM from 'react-dom/client';

// Components
import App from './components/App';

// Styles
import '@fontsource/montserrat';
import './styles/globals.css';
import { AuthContextProvider } from './store/authContext';

const container = document.getElementById('root');
const root = ReactDOM.createRoot(container);

root.render(
  <AuthContextProvider>
    <App />
  </AuthContextProvider>
);
