import { useState } from "react";
import { API_URL, MAX_RETRIES, RETRY_DELAY_MS } from "../../config";
import axios from "axios";
import { wait } from "../../helpers/helpers";
import { useDispatch } from "react-redux";
import { set_username_avatar } from "../../state/username_avatar_slice";
import { UsernameAvatar } from "../../types/types";

const useUsernameAvatarApi = () => {
  const dispatch = useDispatch();
  const [error_data, set_error_data] = useState<string | null>();

  const fetch_data = async (attempts: number = 1) => {
    try {
      const response = await axios.get<UsernameAvatar>(
        `${API_URL}/protected/user/avatar_username`
      );
      dispatch(
        set_username_avatar({
          username: response.data.username,
          avatar: response.data.avatar,
          is_loading: false,
        })
      );
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
                dispatch(
                  set_username_avatar({
                    username: "Username",
                    avatar: "Avatar",
                    is_loading: false,
                  })
                );
              }
              break;
            default:
              console.error(
                "API error: ",
                error.response.status,
                error.response.data
              );
              dispatch(
                set_username_avatar({
                  username: "Username",
                  avatar: "Avatar",
                  is_loading: false,
                })
              );
              break;
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            fetch_data(attempts + 1);
          } else {
            set_error_data(
              "Нет ответа от сервера, пожалуйста, проверьте соединение с интернетом и попробуйте еще раз"
            );
            dispatch(
              set_username_avatar({
                username: "Username",
                avatar: "Avatar",
                is_loading: false,
              })
            );
          }
        } else {
          console.error("API Error: Reqest setup error:", error.message);
          dispatch(
            set_username_avatar({
              username: "Username",
              avatar: "Avatar",
              is_loading: false,
            })
          );
        }
      } else {
        console.error("Non-Axios:", error);
        dispatch(
          set_username_avatar({
            username: "Username",
            avatar: "Avatar",
            is_loading: false,
          })
        );
      }
    }
  };

  return { error_data, get_username_and_avatar: fetch_data };
};

export default useUsernameAvatarApi;
