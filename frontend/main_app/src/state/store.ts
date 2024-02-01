import { configureStore } from "@reduxjs/toolkit";
import active_section_reducer from "./active_section_slice";
import previous_path_reducer from "./previous_path_slice";
import user_permissions_reducer from "./user_permissions_slice";

const store = configureStore({
  reducer: {
    active_section: active_section_reducer,
    previous_path: previous_path_reducer,
    user_permissions: user_permissions_reducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AddDispatch = typeof store.dispatch;
export default store;
