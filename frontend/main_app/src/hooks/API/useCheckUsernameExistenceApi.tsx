import { useState } from "react";
import { API_URL, MAX_RETRIES, RETRY_DELAY_MS } from "../../config";
import axios from "axios";
import { UsernameExistence } from "../../types/types";
import { wait } from "../../helpers/helpers";

const useCheckUsernameExistneceApi = () => {
  const [error_data, set_error_data] = useState<string | null>();

  const fetch_data = async (
    username: string,
    attempts: number = 1
  ): Promise<UsernameExistence | null | undefined> => {
    try {
      const response = await axios.get<UsernameExistence>(
        `${API_URL}/username_status?username=${username}`
      );
      return response.data;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          switch (error.response.status) {
            case 400:
              console.error("Bad request.", error.message);
              break;
            case 500:
              if (attempts < MAX_RETRIES) {
                await wait(RETRY_DELAY_MS);
                return fetch_data(username, attempts + 1);
              } else {
                set_error_data(
                  "Что-то не так с нашим сервером, мы уже работаем над этим. Пожалуйста, попробуйте обновить страницу"
                );
                return null;
              }
            default:
              console.error(
                "API error: ",
                error.response.status,
                error.response.data
              );
              return null;
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            return fetch_data(username, attempts + 1);
          } else {
            console.error("Server is not responding, ", error.message);
            return null;
          }
        } else {
          console.error("API Error: Reqest setup error:", error.message);
          return null;
        }
      } else {
        console.error("Non-Axios:", error);
        return null;
      }
    }
  };

  return { error_data, check_is_username_exists: fetch_data };
};

export default useCheckUsernameExistneceApi;
