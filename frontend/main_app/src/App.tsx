import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.scss";
import MainLayout from "./layouts/MainLayout";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={<MainLayout />}
        >
          <Route
            path="/"
            element={
              <>
                <h1>Home page</h1>
              </>
            }
          />
          <Route
            path="products"
            element={
              <>
                <h1>Products page</h1>
              </>
            }
          />
          <Route
            path="services"
            element={
              <>
                <h1>Services page</h1>
              </>
            }
          />
          <Route
            path="help"
            element={
              <>
                <h1>Help page</h1>
              </>
            }
          />
          <Route
            path="*"
            element={
              <>
                <h1>Page not found</h1>
              </>
            }
          />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
