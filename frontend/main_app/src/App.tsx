import "./App.scss";
import {
  BrowserRouter,
  Navigate,
  Outlet,
  Route,
  Routes,
} from "react-router-dom";
import MainLayout from "./layouts/MainLayout";
import HomePage from "./Pages/Home/HomePage";
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
import Dashboard from "./PersonalAccount/Pages/Dashboard";
import { find_user_role_index, translate_user_role } from "./helpers/helpers";
import AccountProducts from "./PersonalAccount/Pages/AccountProducts";
import UploadNewProduct from "./PersonalAccount/Components/AccountProducts/UploadNewProduct";
import LoadingScreen from "./Components/LoadingScreen";
import UploadProductTempl from "./PersonalAccount/Components/AccountProducts/UploadProductTempl";

function App() {
  const { check_user_permissions } = useCheckPermissionsApi();
  const { get_username_and_avatar } = useUsernameAvatarApi();
  const user_permissions = useSelector(
    (state: RootState) => state.user_permissions
  );
  const username_avatar = useSelector(
    (state: RootState) => state.username_avatar
  );
  // console.log(user_permissions);

  useEffect(() => {
    check_user_permissions();
  }, []);

  useEffect(() => {
    if (
      !user_permissions.is_loading &&
      user_permissions.permissions.length !== 0
    ) {
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
          path="personal-account"
          element={
            user_permissions.is_loading ? (
              <LoadingScreen />
            ) : user_permissions.permissions.length === 0 ? (
              <Navigate to="/login" />
            ) : (
              <PersonalAccount />
            )
          }
        >
          <Route
            path="dashboard"
            element={
              <Dashboard
                username={
                  username_avatar.is_loading ? "..." : username_avatar.username
                }
                user_role={
                  user_permissions.permissions.length !== 0
                    ? user_permissions.is_loading
                      ? "..."
                      : translate_user_role(
                          user_permissions.permissions[
                            find_user_role_index(
                              user_permissions.permissions,
                              "creator"
                            )
                          ].name
                        )
                    : ""
                }
                avatar={
                  username_avatar.is_loading ? "..." : username_avatar.avatar
                }
              />
            }
          />
          <Route
            path="products"
            element={<Outlet />}
          >
            <Route
              index
              element={<AccountProducts />}
            />
            <Route
              path="upload_new_product"
              element={<UploadNewProduct />}
            />
            <Route
              path="upload_song"
              element={<UploadProductTempl kind="song" />}
            >
              <Route
                path="step_1"
                element={<div>step 1</div>}
              />
              <Route
                index
                element={
                  <Navigate
                    to="step_1"
                    replace
                  />
                }
              />
            </Route>
            <Route
              path="upload_beat"
              element={<div>upload beat</div>}
            />
            <Route
              path="upload_text"
              element={<div>upload text</div>}
            />
            <Route
              path="upload_cover"
              element={<div>upload cover</div>}
            />
          </Route>
          <Route
            path="*"
            element={<div>page in development</div>}
          />
          <Route
            index
            element={
              <Navigate
                to="dashboard"
                replace
              />
            }
          />
        </Route>
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
    </BrowserRouter>
  );
}

export default App;
