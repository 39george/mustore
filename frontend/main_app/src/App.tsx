import "./App.scss";
import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import MainLayout from "./layouts/MainLayout";
import HomePage from "./Pages/Home/HomePage";
import Footer from "./Components/Footer";
import ProductsPage from "./Pages/Products/ProductsPage";
import ContentSection from "./Pages/Products/Components/ContentSection";
import SignUp from "./Components/SignUp";
import LogIn from "./Components/LogIn";
import { useEffect } from "react";
import useCheckPermissionsApi from "./hooks/API/useCheckPermissionsApi";
import PersonalAccount from "./PersonalAccount/PersonalAccount";
import useUsernameAvatarApi from "./hooks/API/useUsernameAvatarApi";
import { useSelector } from "react-redux";
import { RootState } from "./state/store";

function App() {
  const { check_user_permissions } = useCheckPermissionsApi();
  const { get_username_and_avatar } = useUsernameAvatarApi();
  const user_permissions = useSelector(
    (state: RootState) => state.user_permissions.permissions
  );

  useEffect(() => {
    check_user_permissions();
  }, []);

  useEffect(() => {
    if (user_permissions.length !== 0) {
      get_username_and_avatar();
    }
  }, [user_permissions]);

  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="signup"
          element={<SignUp />}
        />
        <Route
          path="login"
          element={<LogIn />}
        />
        <Route
          path="personal-account/*"
          element={<PersonalAccount />}
        />
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
                <Footer />
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
                <Footer />
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
                <Footer />
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
                <Footer />
              </div>
            }
          />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
