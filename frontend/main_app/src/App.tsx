import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.scss";
import MainLayout from "./layouts/MainLayout";
import HomePage from "./Pages/Home/HomePage";
import Footer from "./Pages/Footer";

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
            element={<HomePage />}
          />
          <Route
            path="products"
            element={
              <div style={{ height: "100vh" }}>
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  Products page
                </h1>
              </div>
            }
          />
          <Route
            path="services"
            element={
              <div style={{ height: "100vh" }}>
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  Services page
                </h1>
              </div>
            }
          />
          <Route
            path="help"
            element={
              <div style={{ height: "100vh" }}>
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  Help page
                </h1>
              </div>
            }
          />
          <Route
            path="about"
            element={
              <div style={{ height: "100vh" }}>
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  About page
                </h1>
              </div>
            }
          />
          <Route
            path="*"
            element={
              <div style={{ height: "100vh" }}>
                <h1 style={{ width: "fit-content", margin: "0 auto" }}>
                  Page not found
                </h1>
              </div>
            }
          />
        </Route>
      </Routes>
      <Footer />
    </BrowserRouter>
  );
}

export default App;
