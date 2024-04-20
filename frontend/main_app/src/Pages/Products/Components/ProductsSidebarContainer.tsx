import { FC, useEffect, useState } from "react";
import ProductsSidebarComponent from "./ProductsSidebarComponent";
import useGenresMoodsApi from "../../../hooks/API/useGenresMoodsApi";
import useCheckboxState from "../../../hooks/useCheckboxState";
import {
  CheckedItems,
  ExpandedBlocks,
  ExpandingBlocks,
  FilteredResults,
  SearchTerms,
} from "../../../types/types";

interface ProductsSidebarContainerProps {
  main_section_ref: React.RefObject<HTMLDivElement>;
}

type FilteredType = "filtered_genres" | "filtered_moods";

const ProductsSidebarContainer: FC<ProductsSidebarContainerProps> = ({
  main_section_ref,
}) => {
  const [is_small_screen, set_is_small_screen] = useState(
    window.innerWidth <= 768
  );
  const [expanded_blocks, set_expanded_blocks] = useState<ExpandedBlocks>({
    sex: false,
    genres: false,
    tempo: false,
    music_key: false,
    moods: false,
  });
  const {
    data: genres,
    error: genres_error,
    fetch_data: fetch_genres,
  } = useGenresMoodsApi();
  const {
    data: moods,
    error: moods_error,
    fetch_data: fetch_moods,
  } = useGenresMoodsApi();
  const [filtered_results, set_filtered_results] = useState<FilteredResults>({
    filtered_genres: [],
    filtered_moods: [],
  });
  const [search_terms, set_search_terms] = useState<SearchTerms>({
    genres: "",
    moods: "",
  });
  const [checked_sex, set_checked_sex] = useState<CheckedItems>({ any: true });
  const {
    checked_items: checked_genres,
    set_checked_items: set_checked_genres,
    handle_checkbox_change: handle_genres_checkbox_change,
  } = useCheckboxState();
  const {
    checked_items: checked_music_key,
    set_checked_items: set_checked_music_key,
    handle_checkbox_change: handle_music_key_checkbox_change,
  } = useCheckboxState();
  const {
    checked_items: checked_moods,
    set_checked_items: set_checked_moods,
    handle_checkbox_change: handle_moods_checkbox_change,
  } = useCheckboxState();
  const [is_iphone] = useState(/iPhone/.test(navigator.userAgent));

  const define_is_small_screen = () => {
    set_is_small_screen(window.innerWidth <= 768);
  };

  // Changing checkbox for sex block
  const handle_sex_checkbox_change = (sex: string) => {
    set_checked_sex({ [sex]: true });
  };

  // Handle change search terms
  const handle_change_search_terms = (
    e: React.ChangeEvent<HTMLInputElement>,
    name: "genres" | "moods"
  ) => {
    set_search_terms((prev) => ({
      ...prev,
      [name]: e.target.value,
    }));
  };

  // Handle genres/moods search
  const handle_genres_moods_search = (
    kind: string[],
    search_term: string,
    output: FilteredType
  ) => {
    let results = kind.filter((item) =>
      item.toLowerCase().includes(search_term.toLowerCase())
    );
    if (search_term) {
      set_filtered_results((prev) => ({
        ...prev,
        [output]: results,
      }));
    } else {
      set_filtered_results((prev) => ({
        ...prev,
        [output]: kind,
      }));
    }
  };

  // Set all values of a chekced object to `false`
  const set_all_to_false = (
    e: React.MouseEvent<HTMLLIElement>,
    obj: CheckedItems,
    obj_kind: "genres" | "music_key" | "moods"
  ) => {
    e.stopPropagation();

    let new_obj: CheckedItems = {};
    Object.keys(obj).forEach((key) => {
      new_obj[key] = false;
    });

    switch (obj_kind) {
      case "genres":
        set_checked_genres(new_obj);
        break;
      case "music_key":
        set_checked_music_key(new_obj);
        break;
      case "moods":
        set_checked_moods(new_obj);
        break;
    }
  };

  // Expand blocks
  const handle_blocks_expand = (name: ExpandingBlocks) => {
    if (!is_small_screen) {
      return;
    }

    set_expanded_blocks((prev) => ({
      ...prev,
      [name]: !prev[name],
    }));
  };

  // Set initial filtered genres
  useEffect(() => {
    const controller = new AbortController();
    const fetch_genres_moods = async () => {
      try {
        await Promise.all([
          fetch_genres("genres", controller.signal),
          fetch_moods("moods", controller.signal),
        ]);
      } catch (eroor) {
        console.error("An error occured while fetching data");
      }
    };
    fetch_genres_moods();

    return () => {
      if (controller) {
        controller.abort();
      }
    };
  }, []);

  useEffect(() => {
    set_filtered_results({
      filtered_genres: genres,
      filtered_moods: moods,
    });
  }, [genres, moods]);

  // Filter genres or moods
  useEffect(() => {
    if (search_terms.genres || search_terms.genres === "") {
      handle_genres_moods_search(
        genres,
        search_terms.genres,
        "filtered_genres"
      );
    }
    if (search_terms.moods || search_terms.moods === "") {
      handle_genres_moods_search(moods, search_terms.moods, "filtered_moods");
    }
  }, [search_terms]);

  return (
    <ProductsSidebarComponent
      main_section_ref={main_section_ref}
      is_small_screen={is_small_screen}
      define_is_small_screen={define_is_small_screen}
      checked_genres={checked_genres}
      checked_music_key={checked_music_key}
      checked_moods={checked_moods}
      handle_genres_checkbox_change={handle_genres_checkbox_change}
      handle_music_key_checkbox_change={handle_music_key_checkbox_change}
      handle_moods_checkbox_change={handle_moods_checkbox_change}
      filtered_results={filtered_results}
      set_all_to_false={set_all_to_false}
      genres_error={genres_error}
      moods_error={moods_error}
      expanded_blocks={expanded_blocks}
      handle_blocks_expand={handle_blocks_expand}
      checked_sex={checked_sex}
      handle_sex_checkbox_change={handle_sex_checkbox_change}
      search_terms={search_terms}
      handle_change_search_terms={handle_change_search_terms}
      is_iphone={is_iphone}
    />
  );
};

export default ProductsSidebarContainer;
