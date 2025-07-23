import React from 'react';
import ReactDOM from 'react-dom/client';
import '@/style/global.css'; // подключаем Tailwind
import AppRouter from './routes/AppRouter';
import './i18n';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <AppRouter />
  </React.StrictMode>
);