import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export interface UserPermissions {
  name: string;
}

interface InitialState {
  permissions: UserPermissions[];
}

const initialState: InitialState = {
  permissions: [],
};

const user_permissions_slice = createSlice({
  name: "user_permissions",
  initialState,
  reducers: {
    set_user_permissions: (
      state: InitialState,
      action: PayloadAction<UserPermissions[]>
    ) => {
      state.permissions = action.payload;
    },
  },
});

export const { set_user_permissions } = user_permissions_slice.actions;

export default user_permissions_slice.reducer;
