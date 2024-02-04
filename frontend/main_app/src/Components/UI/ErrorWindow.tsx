import styles from "./ErrorWindow.module.scss";
import { FC } from "react";
import { FaCircleXmark } from "react-icons/fa6";
import logo from "../../assets/svg/logo.svg";
import { NavLink } from "react-router-dom";

interface ErrorWindowProps {
  error_message: string;
}
const ErrorWindow: FC<ErrorWindowProps> = ({ error_message }) => {
  return (
    <div className={styles.error_window}>
      <div className={styles.error_container}>
        <div className={styles.content}>
          <FaCircleXmark className={styles.error_icon} />
          <p className={styles.general_info}>Что-то пошло не так...</p>
          <p className={styles.error_message}>{error_message}</p>
          <hr className={styles.divider} />
          <div className={styles.aux_info}>
            <div className={styles.filler}></div>
            <img
              src={logo}
              alt="logo"
              className={styles.logo}
            />
            <NavLink
              to="/"
              className={styles.return_link}
            >
              На главную
            </NavLink>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ErrorWindow;
