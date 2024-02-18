import { useState } from "react";
import { API_URL, MAX_RETRIES, RETRY_DELAY_MS } from "../../config";
import axios from "axios";
import { wait } from "../../helpers/helpers";
import { useDispatch } from "react-redux";
import {
  UserPermissions,
  set_loading_state,
  set_user_permissions,
} from "../../state/user_permissions_slice";

const useCheckPermissionsApi = () => {
  const dispatch = useDispatch();
  const [error_data, set_error_data] = useState<string | null>();

  const fetch_data = async (attempts: number = 1) => {
    try {
      const response = await axios.get<UserPermissions[]>(
        `${API_URL}/protected/user/permissions`
      );
      dispatch(set_loading_state(false));
      dispatch(set_user_permissions(response.data));
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          switch (error.response.status) {
            case 403:
              dispatch(set_loading_state(false));
              dispatch(set_user_permissions([]));
              break;
            case 500:
              if (attempts < MAX_RETRIES) {
                await wait(RETRY_DELAY_MS);
                fetch_data(attempts + 1);
              } else {
                set_error_data(
                  "Что-то не так с нашим сервером, мы уже работаем над этим. Пожалуйста, попробуйте обновить страницу"
                );
                dispatch(set_loading_state(false));
                dispatch(set_user_permissions([]));
              }
              break;
            default:
              console.error(
                "API error: ",
                error.response.status,
                error.response.data
              );
              dispatch(set_loading_state(false));
              dispatch(set_user_permissions([]));
              break;
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            fetch_data(attempts + 1);
          } else {
            dispatch(set_loading_state(false));
            dispatch(set_user_permissions([]));
            set_error_data(
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

  return { error_data, check_user_permissions: fetch_data };
};

export default useCheckPermissionsApi;
