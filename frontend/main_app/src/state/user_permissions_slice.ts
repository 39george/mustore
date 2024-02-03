import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export interface UserPermissions {
  name: string;
}

export interface PermissionsState {
  permissions: UserPermissions[];
  is_loading: boolean;
}
const initialState: PermissionsState = {
  permissions: [],
  is_loading: true,
};

const user_permissions_slice = createSlice({
  name: "user_permissions",
  initialState,
  reducers: {
    set_user_permissions: (
      state: PermissionsState,
      action: PayloadAction<UserPermissions[]>
    ) => {
      state.permissions = action.payload;
    },
    set_loading_state: (
      state: PermissionsState,
      action: PayloadAction<boolean>
    ) => {
      state.is_loading = action.payload;
    },
  },
});

export const { set_user_permissions, set_loading_state } =
  user_permissions_slice.actions;

export default user_permissions_slice.reducer;
