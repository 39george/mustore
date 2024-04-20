import styles from "./MainContentProducts.module.scss";
import { FC, useRef } from "react";
import { song_items } from "./UI/content_dummies";
import SongItem from "./UI/SongItem";
import ProductsSidebarContainer from "./ProductsSidebarContainer";

const MainContentProducts: FC = () => {
  const main_section_ref = useRef<HTMLDivElement>(null);

  return (
    <div
      ref={main_section_ref}
      className={styles.main_seciton}
    >
      <ProductsSidebarContainer main_section_ref={main_section_ref} />
      <div className={styles.products_container}>
        {song_items.map((item) => {
          return (
            <SongItem
              key={item.id}
              name={item.name}
              cover_url={item.cover_url}
              author={item.author}
              price={item.price}
              primary_genre={item.primary_genre}
              moods={item.moods}
            />
          );
        })}
      </div>
    </div>
  );
};

export default MainContentProducts;
