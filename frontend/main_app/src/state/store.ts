import { configureStore } from "@reduxjs/toolkit";
import active_section_reducer from "./active_section_slice";
import previous_path_reducer from "./previous_path_slice";
import user_permissions_reducer from "./user_permissions_slice";
import sidebar_actions_reducer from "./sidebar_actions_slice";
import username_avatar_reducer from "./username_avatar_slice";
import product_status_reducer from "./product_status_slice";

const store = configureStore({
  reducer: {
    active_section: active_section_reducer,
    previous_path: previous_path_reducer,
    user_permissions: user_permissions_reducer,
    sidebar_actions: sidebar_actions_reducer,
    username_avatar: username_avatar_reducer,
    product_status: product_status_reducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AddDispatch = typeof store.dispatch;
export default store;
