import styles from "./PersonalAccount.module.scss";
import {
  BrowserRouter,
  Link,
  Navigate,
  Outlet,
  Route,
  Routes,
} from "react-router-dom";
import { FC } from "react";
import Sidebar from "./Components/Sidebar";
import TopBar from "./Components/TopBar";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import Dashboard from "./Pages/Dashboard";
import { find_user_role_index, translate_user_role } from "../helpers/helpers";
import AccountProducts from "./Pages/AccountProducts";
import UploadNewProduct from "./Components/UI_AccountProducts/UploadNewProduct";

const PersonalAccount: FC = () => {
  const username_avatar = useSelector(
    (state: RootState) => state.username_avatar
  );
  const user_permissions = useSelector(
    (state: RootState) => state.user_permissions
  );

  return (
    <div className={styles.personal_account}>
      <Sidebar
        username={username_avatar.is_loading ? "..." : username_avatar.username}
        user_role={
          user_permissions.is_loading
            ? "..."
            : translate_user_role(
                user_permissions.permissions[
                  find_user_role_index(user_permissions.permissions, "creator")
                ].name
              )
        }
        avatar={username_avatar.is_loading ? "..." : username_avatar.avatar}
      />
      <div className={styles.main_view}>
        <TopBar
          username={
            username_avatar.is_loading ? "..." : username_avatar.username
          }
          avatar={username_avatar.is_loading ? "..." : username_avatar.avatar}
        />
        <hr className={styles.top_border} />
        <Routes>
          <Route
            path="dashboard"
            element={
              <Dashboard
                username={
                  username_avatar.is_loading ? "..." : username_avatar.username
                }
                user_role={
                  user_permissions.is_loading
                    ? "..."
                    : translate_user_role(
                        user_permissions.permissions[
                          find_user_role_index(
                            user_permissions.permissions,
                            "creator"
                          )
                        ].name
                      )
                }
                avatar={
                  username_avatar.is_loading ? "..." : username_avatar.avatar
                }
              />
            }
          />
          <Route
            path="products"
            element={<AccountProducts />}
          />
          <Route
            path="products/upload_new_product"
            element={<UploadNewProduct />}
          />
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
        </Routes>
      </div>
      <div className={styles.filler_main}></div>
    </div>
  );
};

export default PersonalAccount;
