import { LoginPage } from "./views/login-page";
import { Provider as AuthProvider } from "./lib/auth/provider";
import { createRoot } from "react-dom/client";

const App = () => {
  return (
    <AuthProvider>
      <LoginPage />
    </AuthProvider>
  );
};

const root = document.getElementById("app");

if (!root) {
  throw new Error("Root element not found");
}

createRoot(root).render(<App />);
