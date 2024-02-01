import axios from "axios";
import { API_URL, MAX_RETRIES, RETRY_DELAY_MS } from "../config";
import { wait } from "../helpers/helpers";
import { useState } from "react";

const useSignUpUserApi = () => {
  const [signup_status, set_signup_status] = useState<number | null>(null);
  const [signup_error, set_signup_error] = useState<string | null>(null);

  const post_data = async (data: string, attempts: number = 1) => {
    try {
      const response = await axios.post(`${API_URL}/signup`, data, {
        headers: {
          "Content-Type": "application/x-www-form-urlencoded",
        },
      });
      set_signup_status(response.status);
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          switch (error.response.status) {
            case 400:
              console.error("Bad request.", error.message);
              break;
            case 403:
              set_signup_error("Recaptcha verification failed");
              break;
            case 500:
              if (attempts < MAX_RETRIES) {
                await wait(RETRY_DELAY_MS);
                post_data(data, attempts + 1);
              } else {
                set_signup_error(
                  "Что-то не так с нашим сервером, мы уже работаем над этим. Пожалуйста, попробуйте обновить страницу"
                );
              }
              break;
            default:
              console.error(
                "API error: ",
                error.response.status,
                error.response.data
              );
              set_signup_error(
                "Нет ответа от сервера, пожалуйста, проверьте соединение с интернетом и попробуйте еще раз"
              );
              break;
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            post_data(data, attempts + 1);
          } else {
            set_signup_error(
              "Нет ответа от сервера, пожалуйста, проверьте соединение с интернетом и попробуйте еще раз"
            );
          }
        } else {
          console.error("API Error: Reqest setup error:", error.message);
        }
      } else {
        console.error("Non-Axios:", error);
      }
    }
  };

  return { signup_status, signup_error, post_data };
};

export default useSignUpUserApi;
