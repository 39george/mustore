import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { UsernameAvatar } from "../types/types";

const initialState: UsernameAvatar = {
  username: "",
  avatar: "",
};

const username_avatar_slice = createSlice({
  name: "username_avatar",
  initialState: initialState,
  reducers: {
    set_username: (state: UsernameAvatar, action: PayloadAction<string>) => {
      state.username = action.payload;
    },
    set_avatar: (state: UsernameAvatar, action: PayloadAction<string>) => {
      state.avatar = action.payload;
    },
  },
});

export const { set_username, set_avatar } = username_avatar_slice.actions;

export default username_avatar_slice.reducer;
