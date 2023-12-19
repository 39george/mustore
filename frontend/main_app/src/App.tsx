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
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  Home page
                </h1>
              </>
            }
          />
          <Route
            path="products"
            element={
              <>
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  Products page
                </h1>
              </>
            }
          />
          <Route
            path="services"
            element={
              <>
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  Services page
                </h1>
              </>
            }
          />
          <Route
            path="help"
            element={
              <>
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  Help page
                </h1>
              </>
            }
          />
          <Route
            path="*"
            element={
              <>
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  Page not found
                </h1>
              </>
            }
          />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
