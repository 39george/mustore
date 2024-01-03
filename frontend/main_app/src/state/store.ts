import { configureStore } from "@reduxjs/toolkit";
import active_section_reducer from "./active_section_slice";

const store = configureStore({
  reducer: {
    active_section: active_section_reducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AddDispatch = typeof store.dispatch;
export default store;
