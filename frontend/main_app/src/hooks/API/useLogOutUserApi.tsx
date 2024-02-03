import { useState } from "react";
import { API_URL, MAX_RETRIES, RETRY_DELAY_MS } from "../../config";
import axios from "axios";
import { UsernameExistence } from "../../types/types";
import { wait } from "../../helpers/helpers";

const useLogOutUserApi = () => {
  const [error_data, set_error_data] = useState<string | null>();

  const fetch_data = async (attempts: number = 1) => {
    try {
      const response = await axios.get<UsernameExistence>(`${API_URL}/logout`);
      console.log(response.status);
      // if (response.status === 303) {
      //   console.log("logout success");
      // }
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          switch (error.response.status) {
            case 500:
              if (attempts < MAX_RETRIES) {
                await wait(RETRY_DELAY_MS);
                fetch_data(attempts + 1);
              } else {
                set_error_data(
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
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            fetch_data(attempts + 1);
          } else {
            console.error("Server is not responding, ", error.message);
          }
        } else {
          console.error("API Error: Reqest setup error:", error.message);
        }
      } else {
        console.error("Non-Axios:", error);
      }
    }
  };

  return { error_data, logout: fetch_data };
};

export default useLogOutUserApi;
