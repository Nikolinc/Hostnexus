import Dashboard from "../pages/Dashboard";
import Login from "../pages/Login";


export const routes = [
  {
    path: "/login",
    element: <Login />,
    public: true,
  },
  {
    path: "/",
    element: <Dashboard />,
    public: false,
  },
];
