import { useLocation } from "react-router-dom";
import { page_names } from "../types/types";

const usePageNavigation = () => {
  const location = useLocation();

  const handle_page_navigation = (page_name: page_names): void => {
    if (location.pathname === `/${page_name}`) {
      window.scrollTo({ top: 0, behavior: "smooth" });
    } else {
      window.scrollTo({ top: 0, behavior: "auto" });
    }
  };

  return handle_page_navigation;
};

export default usePageNavigation;
