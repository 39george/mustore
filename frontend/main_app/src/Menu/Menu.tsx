import { NavLink } from "react-router-dom";
import styles from "./Menu.module.scss";

const Menu = () => {
  return (
    <nav className={styles.nav_bar}>
      <NavLink
        to="."
        end
      >
        logo
      </NavLink>
      <NavLink to="products">Товары</NavLink>
      <NavLink to="services">Услуги</NavLink>
      <NavLink to="help">Помощь</NavLink>
      <div>
        <div>войти</div>
        <div>|</div>
        <div>создать аккаунт</div>
      </div>
    </nav>
  );
};

export default Menu;
