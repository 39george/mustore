import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import "./App.scss";
import MainLayout from "./layouts/MainLayout";
import HomePage from "./Pages/Home/HomePage";
import Footer from "./Pages/Footer";
import ProductsPage from "./Pages/Products/ProductsPage";
import ContentSection from "./Pages/Products/Components/ContentSection";
import SignUp from "./Components/SignUp";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={<MainLayout />}
        >
          <Route
            path="signup"
            element={<SignUp />}
          />
          <Route
            path="/"
            element={<HomePage />}
          />
          <Route
            path="products"
            element={<ProductsPage />}
          >
            <Route
              path="beats"
              element={<ContentSection section_type="beats" />}
            />
            <Route
              path="covers"
              element={<ContentSection section_type="covers" />}
            />
            <Route
              path="songs"
              element={<ContentSection section_type="songs" />}
            />
            <Route
              path="texts"
              element={<ContentSection section_type="texts" />}
            />
            <Route
              index
              element={
                <Navigate
                  to="songs"
                  replace
                />
              }
            />
          </Route>
          <Route
            path="services"
            element={
              <div>
                <h1 style={{ width: "fit-content", margin: "6rem auto 0" }}>
                  Services page
                </h1>
                <div style={{ height: "100vh" }}></div>
              </div>
            }
          />
          <Route
            path="help"
            element={
              <div>
                <h1 style={{ width: "fit-content", margin: "6rem auto 0" }}>
                  Help page
                </h1>
                <div style={{ height: "100vh" }}></div>
              </div>
            }
          />
          <Route
            path="about"
            element={
              <div>
                <h1 style={{ width: "fit-content", margin: "6rem auto 0" }}>
                  About page
                </h1>
                <div style={{ height: "100vh" }}></div>
              </div>
            }
          />
          <Route
            path="*"
            element={
              <div>
                <h1 style={{ width: "fit-content", margin: "6rem auto 0" }}>
                  Page not found
                </h1>
                <div style={{ height: "100vh" }}></div>
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
