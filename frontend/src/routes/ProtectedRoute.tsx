import type { JSX } from "react";
import { Navigate } from "react-router-dom";

const isAuthenticated = () => {
  // Пока заглушка. Потом заменим на проверку токена или состояния пользователя
  return false;
};

const ProtectedRoute = ({ children }: { children: JSX.Element }) => {
  if (!isAuthenticated()) {
    return <Navigate to="/login" replace />;
  }
  return children;
};

export default ProtectedRoute;
