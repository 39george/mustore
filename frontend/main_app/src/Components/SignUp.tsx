import styles from "./SignUp.module.scss";
import { FC, FormEvent, useEffect, useState } from "react";
import { HiMiniXMark } from "react-icons/hi2";
import { useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { z } from "zod";
import zxcvbn from "zxcvbn";
import { RootState } from "../state/store";
import { FaEye } from "react-icons/fa";
import { TbEyeClosed } from "react-icons/tb";
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

type PasswordStatus =
  | ""
  | "пароль не должен содержать пробелов"
  | "очень слабый"
  | "слабый"
  | "предсказуемый"
  | "ненадежный"
  | "надежный";

const colors = {
  warning: "#EF0606",
  neutral: "#d9d9d9",
  password_too_weak: "#700000",
  password_middle: "#fe8c49",
  password_unreliable: "#E6A600",
  success: "#599c00",
};

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
  const [password_status, set_password_status] = useState<PasswordStatus>("");
  const [email_input_info, set_email_input_info] = useState<EmailInputInfo>({});
  const email_schema = z.string().email();
  const [password_status_bar_colors, set_password_status_bar_colors] = useState(
    {
      bar_1: colors.neutral,
      bar_2: colors.neutral,
      bar_3: colors.neutral,
      bar_4: colors.neutral,
      bar_5: colors.neutral,
    }
  );
  const [password_status_color, set_password_status_color] = useState("");
  const [input_validity, set_input_validity] = useState({
    username: false,
    email: false,
    password: false,
    account_type: false,
  });

  const handle_change = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    console.log(e.target);
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
        set_input_validity((prev) => ({
          ...prev,
          email: true,
        }));
      } else {
        set_email_input_info({
          success: false,
          message: "некорректный email",
        });
        set_input_validity((prev) => ({
          ...prev,
          email: false,
        }));
      }
    }
  }, [form_data.email]);

  // Checking password strenght
  useEffect(() => {
    if (form_data.password !== "") {
      if (form_data.password.includes(" ")) {
        set_password_status("пароль не должен содержать пробелов");
        set_password_status_color(colors.warning);
      } else {
        let user_inputs: string[] = [];
        user_inputs.push(form_data.username);
        user_inputs.push(form_data.email);

        const password_strenght_info = zxcvbn(form_data.password, user_inputs);
        switch (password_strenght_info.score) {
          case 0:
            set_password_status("очень слабый");
            set_password_status_color(colors.password_too_weak);
            set_password_status_bar_colors({
              bar_1: colors.password_too_weak,
              bar_2: colors.neutral,
              bar_3: colors.neutral,
              bar_4: colors.neutral,
              bar_5: colors.neutral,
            });
            break;
          case 1:
            set_password_status("слабый");
            set_password_status_color(colors.warning);
            set_password_status_bar_colors({
              bar_1: colors.warning,
              bar_2: colors.warning,
              bar_3: colors.neutral,
              bar_4: colors.neutral,
              bar_5: colors.neutral,
            });
            break;
          case 2:
            set_password_status("предсказуемый");
            set_password_status_color(colors.password_middle);
            set_password_status_bar_colors({
              bar_1: colors.password_middle,
              bar_2: colors.password_middle,
              bar_3: colors.password_middle,
              bar_4: colors.neutral,
              bar_5: colors.neutral,
            });
            break;
          case 3:
            set_password_status("ненадежный");
            set_password_status_color(colors.password_unreliable);
            set_password_status_bar_colors({
              bar_1: colors.password_unreliable,
              bar_2: colors.password_unreliable,
              bar_3: colors.password_unreliable,
              bar_4: colors.password_unreliable,
              bar_5: colors.neutral,
            });
            break;
          case 4:
            set_password_status("надежный");
            set_password_status_color(colors.success);
            set_password_status_bar_colors({
              bar_1: colors.success,
              bar_2: colors.success,
              bar_3: colors.success,
              bar_4: colors.success,
              bar_5: colors.success,
            });
            break;
        }
      }
    } else {
      set_password_status("");
    }
  }, [form_data.password]);

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
                  className={styles.sign_up_input}
                  style={{
                    border: `${
                      form_data.email !== ""
                        ? email_input_info.success
                          ? `1px solid ${colors.success}`
                          : `1px solid ${colors.warning}`
                        : ""
                    }`,
                  }}
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
                  style={{
                    border: `${
                      password_status !== ""
                        ? `1px solid ${password_status_color}`
                        : ""
                    }`,
                  }}
                  required
                />
                <div
                  className={styles.eye_icon}
                  onClick={() => on_eye_click("password")}
                >
                  {is_password_visible.password ? <FaEye /> : <TbEyeClosed />}
                </div>
              </div>
              {password_status !== "" && (
                <>
                  <div className={styles.input_info}>
                    <div>
                      {password_status === "надежный" ? (
                        <GoCheckCircleFill
                          className={styles.success_icon}
                          style={{ color: `${password_status_color}` }}
                        />
                      ) : (
                        <FaTriangleExclamation
                          className={styles.warning_icon}
                          style={{ color: `${password_status_color}` }}
                        />
                      )}
                    </div>
                    <p
                      className={styles.info_message}
                      style={{ color: `${password_status_color}` }}
                    >
                      {password_status}
                    </p>
                  </div>
                  {password_status !==
                    "пароль не должен содержать пробелов" && (
                    <div className={styles.password_status_bar}>
                      <div
                        className={styles.bar}
                        style={{
                          backgroundColor: `${password_status_bar_colors.bar_1}`,
                        }}
                      ></div>
                      <div
                        className={styles.bar}
                        style={{
                          backgroundColor: `${password_status_bar_colors.bar_2}`,
                        }}
                      ></div>
                      <div
                        className={styles.bar}
                        style={{
                          backgroundColor: `${password_status_bar_colors.bar_3}`,
                        }}
                      ></div>
                      <div
                        className={styles.bar}
                        style={{
                          backgroundColor: `${password_status_bar_colors.bar_4}`,
                        }}
                      ></div>
                      <div
                        className={styles.bar}
                        style={{
                          backgroundColor: `${password_status_bar_colors.bar_5}`,
                        }}
                      ></div>
                    </div>
                  )}
                </>
              )}
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
                  {is_password_visible.confirm_password ? (
                    <FaEye />
                  ) : (
                    <TbEyeClosed />
                  )}
                </div>
              </div>
            </div>
            {/* <div className={styles.input_block}>
              <div
                className={`${styles.input_container} ${styles.account_types}`}
              >
                <div className={styles.account_type_header}>
                  <p className={styles.account_type_p}>Выберите тип аккаунта</p>
                  <GoChevronDown className={styles.chevron_icon} />
                </div>
                <div className={styles.account_types_container}>
                  <p className={styles.account_type}>Автор</p>
                  <p className={styles.account_type}>Покупатель</p>
                </div>
              </div>
            </div> */}
            <button type="submit">Присоединиться</button>
          </form>
        </div>
      </div>
    </div>
  );
};

export default SignUp;
