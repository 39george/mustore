import { configureStore } from "@reduxjs/toolkit";
import active_section_reducer from "./active_section_slice";
import previous_path_reducer from "./previous_path_slice";
import user_permissions_reducer from "./user_permissions_slice";
import sidebar_collapsed_reducer from "./sidebar_collapsed_slice";

const store = configureStore({
  reducer: {
    active_section: active_section_reducer,
    previous_path: previous_path_reducer,
    user_permissions: user_permissions_reducer,
    sidebar_collapsed: sidebar_collapsed_reducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AddDispatch = typeof store.dispatch;
export default store;
