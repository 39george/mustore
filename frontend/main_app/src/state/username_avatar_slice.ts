import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { UsernameAvatar } from "../types/types";

const initialState: UsernameAvatar = {
  username: "",
  avatar: "",
  is_loading: true,
};

const username_avatar_slice = createSlice({
  name: "username_avatar",
  initialState: initialState,
  reducers: {
    set_username_avatar: (
      state: UsernameAvatar,
      action: PayloadAction<UsernameAvatar>
    ) => {
      state.username = action.payload.username;
      state.avatar = action.payload.avatar;
      state.is_loading = action.payload.is_loading;
    },
  },
});

export const { set_username_avatar } = username_avatar_slice.actions;

export default username_avatar_slice.reducer;
