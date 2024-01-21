import styles from "./SignUp.module.scss";
import { FC, FormEvent, useEffect, useState } from "react";
import { HiMiniXMark } from "react-icons/hi2";
import { useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { z } from "zod";
import zxcvbn from "zxcvbn";
import { RootState } from "../state/store";
import { FaEye, FaEyeSlash } from "react-icons/fa";
import { FaTriangleExclamation } from "react-icons/fa6";
import { GoCheckCircleFill } from "react-icons/go";

interface FromData {
  username: string;
  email: string;
  password: string;
  confirm_password: string;
}

type InputName = "password" | "confirm_password";

type InputNames = {
  [key in InputName]: boolean;
};

type InputType = "text" | "password";

type InputTypes = {
  [key in InputName]: InputType;
};

interface EmailInputInfo {
  success?: boolean;
  message?: string;
}

const SignUp: FC = () => {
  const navigate = useNavigate();
  const previous_path = useSelector<RootState, string>(
    (state) => state.previous_path.previous_path
  );
  const [form_data, set_form_data] = useState<FromData>({
    username: "",
    email: "",
    password: "",
    confirm_password: "",
  });
  const [input_type, set_input_type] = useState<InputTypes>({
    password: "password",
    confirm_password: "password",
  });
  const [is_password_visible, set_is_password_visible] = useState<InputNames>({
    password: false,
    confirm_password: false,
  });
  const [email_input_info, set_email_input_info] = useState<EmailInputInfo>({});
  const [email_validation_class_name, set_email_validation_class_name] =
    useState("");
  const email_schema = z.string().email();

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

  // Checking email validity
  useEffect(() => {
    if (form_data.email !== "") {
      const validation_info = email_schema.safeParse(form_data.email);
      if (validation_info.success) {
        set_email_input_info({
          success: true,
          message: "корректный email",
        });
      } else {
        set_email_input_info({
          success: false,
          message: "некорректный email",
        });
      }
    }
  }, [form_data.email]);

  useEffect(() => {
    if (form_data.email !== "") {
      if (email_input_info.success === true) {
        set_email_validation_class_name(`${styles.sign_up_input_success}`);
      } else if (email_input_info.success === false) {
        set_email_validation_class_name(`${styles.sign_up_input_warning}`);
      }
    } else {
      set_email_validation_class_name("");
    }
  }, [email_input_info, form_data.email]);

  // Handling returning to the previous page
  const handle_close = () => {
    navigate(previous_path);
  };

  // Handling click on the eye icon
  const on_eye_click = (input_name: InputName) => {
    set_is_password_visible((prev) => ({
      ...prev,
      [input_name]: !prev[input_name],
    }));
  };

  useEffect(() => {
    if (is_password_visible.password) {
      set_input_type((prev) => ({
        ...prev,
        password: "text",
      }));
    } else {
      set_input_type((prev) => ({
        ...prev,
        password: "password",
      }));
    }

    if (is_password_visible.confirm_password) {
      set_input_type((prev) => ({
        ...prev,
        confirm_password: "text",
      }));
    } else {
      set_input_type((prev) => ({
        ...prev,
        confirm_password: "password",
      }));
    }
  }, [is_password_visible.password, is_password_visible.confirm_password]);

  // Rendering component
  return (
    <div className={styles.sign_up_window}>
      <HiMiniXMark
        className={styles.close_icon}
        onClick={handle_close}
      />
      <div className={styles.log_in_section}>
        <div className={styles.log_in_content}>
          <p className={styles.p_log_in}>Уже есть аккаунт?</p>
          <button className={styles.button_log_in}>Войти</button>
        </div>
      </div>
      <div className={styles.sign_up_section}>
        <div className={styles.sign_up_content}>
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
            <div className={styles.input_block}>
              <div className={styles.input_container}>
                <input
                  type="text"
                  name="username"
                  placeholder="Имя пользователя"
                  onChange={handle_change}
                  className={styles.sign_up_input}
                  required
                />
              </div>
            </div>
            <div className={styles.input_block}>
              <div className={styles.input_container}>
                <input
                  type="text"
                  name="email"
                  onChange={handle_change}
                  placeholder="Email"
                  className={`${styles.sign_up_input} ${email_validation_class_name}`}
                  required
                />
              </div>
              {form_data.email !== "" && (
                <div className={styles.input_info}>
                  <div>
                    {email_input_info.success ? (
                      <GoCheckCircleFill className={styles.success_icon} />
                    ) : (
                      <FaTriangleExclamation className={styles.warning_icon} />
                    )}
                  </div>
                  <p
                    className={`${styles.info_message} ${
                      email_input_info.success
                        ? styles.info_success
                        : styles.info_warning
                    }`}
                  >
                    {email_input_info.message}
                  </p>
                </div>
              )}
            </div>
            <div className={styles.input_block}>
              <div className={styles.input_container}>
                <input
                  type={input_type.password}
                  name="password"
                  onChange={handle_change}
                  placeholder="Пароль"
                  className={styles.sign_up_input}
                  required
                />
                <div
                  className={styles.eye_icon}
                  onClick={() => on_eye_click("password")}
                >
                  {!is_password_visible.password ? <FaEye /> : <FaEyeSlash />}
                </div>
              </div>
            </div>
            <div className={styles.input_block}>
              <div className={styles.input_container}>
                <input
                  type={input_type.confirm_password}
                  name="confirm_password"
                  onChange={handle_change}
                  placeholder="Подтвердите пароль"
                  className={styles.sign_up_input}
                  required
                />
                <div
                  className={styles.eye_icon}
                  onClick={() => on_eye_click("confirm_password")}
                >
                  {!is_password_visible.confirm_password ? (
                    <FaEye />
                  ) : (
                    <FaEyeSlash />
                  )}
                </div>
              </div>
            </div>
            <button type="submit">Присоединиться</button>
          </form>
        </div>
      </div>
    </div>
  );
};

export default SignUp;
