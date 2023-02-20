import { LoginPage } from "./views/login-page";
import { Provider as AuthProvider } from "./lib/auth/provider";
import { createRoot } from "react-dom/client";

const App = () => {
  return (
    <AuthProvider>
      <LoginPage />
    </AuthProvider>
  );
}

createRoot(document.getElementById("app"))
  .render(<App />);
