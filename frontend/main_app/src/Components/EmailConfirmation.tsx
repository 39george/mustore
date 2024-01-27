import styles from "./EmailConfirmation.module.scss";
import { NavLink } from "react-router-dom";
import logo from "../assets/svg/logo.svg";
import { FC } from "react";
import { useDispatch } from "react-redux";
import { set_active_section } from "../state/active_section_slice";

const EmailConfirmation: FC = () => {
  const dispatch = useDispatch();
  const handle_click = () => {
    dispatch(set_active_section(null));
  };

  return (
    <div className={styles.email_confirmation}>
      <div className={styles.confirmation_container}>
        <div className={styles.content}>
          <h2 className={styles.h2}>Поздравляем!</h2>
          <p className={styles.p}>
            Вы успешно создали аккаунт на{" "}
            <span className={styles.harmonysphere}>
              HARMONY<span>.</span>SPHERE
            </span>
            <br />
            <br />
            На ваш email было отправлено письмо для подтверждения аккаунта.
            Пожалуйста, перейдите по ссылке из этого письма, чтобы завершить
            регистрацию.
          </p>
          <hr className={styles.divider} />
          <div className={styles.aux_info}>
            <NavLink
              to="../help"
              className={styles.help_link}
              onClick={handle_click}
            >
              Не приходит письмо?
            </NavLink>
            <img
              src={logo}
              alt="logo"
              className={styles.logo}
            />
            <div
              style={{
                width: "6.875rem",
                height: "1rem",
              }}
            ></div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default EmailConfirmation;
