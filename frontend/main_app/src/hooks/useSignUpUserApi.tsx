import axios from "axios";
import { API_URL, MAX_RETRIES, RETRY_DELAY_MS } from "../config";
import { handle_axios_error, wait } from "../helpers/helpers";
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
          console.error(
            "API Error:",
            error.response.status,
            error.response.data
          );

          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            post_data(data, attempts + 1);
          } else {
            handle_axios_error(error, set_signup_error);
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
