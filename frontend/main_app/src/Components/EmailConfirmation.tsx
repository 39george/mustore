import styles from "./EmailConfirmation.module.scss";
import { NavLink } from "react-router-dom";
import logo from "../assets/svg/logo.svg";
import { FC } from "react";

const EmailConfirmation: FC = () => {
  return (
    <div className={styles.eamil_confirmation}>
      <h2 className={styles.h2}>Поздравляем!</h2>
      <p className={styles.p}>
        Вы успешно создали аккаунт на{" "}
        <span className={styles.harmonyshpere}>
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
          to="help"
          className={styles.help_link}
        >
          Не приходит письмо?
        </NavLink>
        <img
          src={logo}
          alt="logo"
          className={styles.logo}
        />
      </div>
    </div>
  );
};

export default EmailConfirmation;
