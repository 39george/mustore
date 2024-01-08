import styles from "./SignUp.module.scss";
import { FC, FormEvent, useState } from "react";
import { HiMiniXMark } from "react-icons/hi2";
import { NavLink } from "react-router-dom";

interface FromData {
  username: string;
  email: string;
  password: string;
  confirm_password: string;
}

const SignUp: FC = () => {
  const [form_data, set_form_data] = useState<FromData>({
    username: "",
    email: "",
    password: "",
    confirm_password: "",
  });

  const handle_change = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    set_form_data((prev_state) => ({
      ...prev_state,
      [name]: value,
    }));
  };

  const handle_submit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    console.log(form_data);
  };

  return (
    <div className={styles.sign_up_window}>
      <NavLink to="/">
        <HiMiniXMark className={styles.close_icon} />
      </NavLink>
      <div className={styles.log_in_section}>
        <p className={styles.p_log_in}>Уже есть аккаунт?</p>
        <button className={styles.button_log_in}>Войти</button>
      </div>
      <div className={styles.sign_up_section}>
        <p className={styles.p_join_us}>присоединяйтесь к сообществу</p>
        <h1 className={styles.title}>
          HARMONY<span>.</span>
          <br />
          SPHERE
        </h1>
        <h2 className={styles.tagline}>
          Сервис для тех, кто хочет найти или предложить свои{" "}
          <span>музыкальные решения.</span>
        </h2>
        <form
          onSubmit={handle_submit}
          className={styles.sign_up_form}
        >
          <input
            type="text"
            name="username"
            value={form_data.username}
            onChange={handle_change}
            placeholder="Имя пользователя"
            className={styles.sign_up_input}
            required
          />
          <input
            type="text"
            name="email"
            value={form_data.email}
            onChange={handle_change}
            placeholder="Email"
            className={styles.sign_up_input}
            required
          />
          <input
            type="password"
            name="password"
            value={form_data.password}
            onChange={handle_change}
            placeholder="Пароль"
            className={styles.sign_up_input}
            required
          />
          <input
            type="password"
            name="confirm_password"
            value={form_data.confirm_password}
            onChange={handle_change}
            placeholder="Подтвердите пароль"
            className={styles.sign_up_input}
            required
          />
          <button type="submit">Присоединиться</button>
        </form>
      </div>
    </div>
  );
};

export default SignUp;
