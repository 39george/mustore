import { useLocation } from "react-router-dom";
import { page_names } from "../types/types";
import { useDispatch } from "react-redux";
import {
  ActiveSection,
  set_active_section,
} from "../state/active_section_slice";

const usePageNavigation = () => {
  const location = useLocation();
  const dispatch = useDispatch();

  const change_section = (section: ActiveSection) => {
    dispatch(set_active_section(section));
  };

  const handle_page_navigation = (page_name: page_names): void => {
    if (location.pathname === `/${page_name}`) {
      window.scrollTo({ top: 0, behavior: "smooth" });
    } else {
      window.scrollTo({ top: 0, behavior: "auto" });
    }

    change_section("hero");
  };

  return handle_page_navigation;
};

export default usePageNavigation;
