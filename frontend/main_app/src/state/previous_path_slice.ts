import { PayloadAction, createSlice } from "@reduxjs/toolkit";

interface PreviousPath {
  previous_path: string;
}

const initialState: PreviousPath = {
  previous_path: "/",
};

const previous_path_slice = createSlice({
  name: "previous_path",
  initialState,
  reducers: {
    set_previous_path: (state: PreviousPath, action: PayloadAction<string>) => {
      state.previous_path = action.payload;
    },
  },
});

export const { set_previous_path } = previous_path_slice.actions;

export default previous_path_slice.reducer;
