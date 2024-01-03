import { FC, useEffect, useRef } from "react";
import AuthorsReviews from "./Components/AuthorsReviews";
import Hero from "./Components/Hero";
import JoinUs from "./Components/JoinUs";
import Products from "./Components/Products";
import Services from "./Components/Services";
import WhyUs from "./Components/WhyUs";
import { useDispatch } from "react-redux";
import {
  ActiveSection,
  set_active_section,
} from "../../state/active_section_slice";

interface HomePageRefs {
  hero_ref: React.RefObject<HTMLDivElement>;
  why_us_ref: React.RefObject<HTMLDivElement>;
  group_ref: React.RefObject<HTMLDivElement>;
  products_ref: React.RefObject<HTMLDivElement>;
  services_ref: React.RefObject<HTMLDivElement>;
  join_us_ref: React.RefObject<HTMLDivElement>;
  authors_reviews_ref: React.RefObject<HTMLDivElement>;
}

interface CurrentEntries {
  id: ActiveSection;
  is_intersecting: boolean;
}

const HomePage: FC = () => {
  const refs: HomePageRefs = {
    hero_ref: useRef(null),
    why_us_ref: useRef(null),
    group_ref: useRef(null),
    products_ref: useRef(null),
    services_ref: useRef(null),
    join_us_ref: useRef(null),
    authors_reviews_ref: useRef(null),
  };

  let current_entries = useRef<CurrentEntries[]>([
    {
      id: "hero",
      is_intersecting: false,
    },
    {
      id: "why_us",
      is_intersecting: false,
    },
    {
      id: "group",
      is_intersecting: false,
    },
    {
      id: "authors_reviews",
      is_intersecting: false,
    },
  ]);
  let prioritized_section = useRef<CurrentEntries>({
    id: null,
    is_intersecting: true,
  });
  const dispatch = useDispatch();

  const change_section = (section: ActiveSection) => {
    dispatch(set_active_section(section));
  };

  useEffect(() => {
    const observings = [
      refs.hero_ref.current,
      refs.why_us_ref.current,
      refs.group_ref.current,
      refs.authors_reviews_ref.current,
    ];

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          let changed_entry = current_entries.current.find((obj) => {
            return obj.id === entry.target.id;
          });
          if (changed_entry) {
            changed_entry.is_intersecting = entry.isIntersecting;
          }
        });
        let currently_intersecting = current_entries.current.filter((obj) => {
          return obj.is_intersecting === true;
        });
        if (currently_intersecting[0].id !== prioritized_section.current.id) {
          prioritized_section.current.id = currently_intersecting[0].id;
          change_section(prioritized_section.current.id);
        }
      },
      {
        threshold: 0.04,
      }
    );

    observings.forEach((ref) => {
      if (ref) {
        observer.observe(ref);
      }
    });
  }, []);

  const scroll_to_why_us = () => {
    refs.why_us_ref.current?.scrollIntoView({ behavior: "smooth" });
  };

  // useEffect(() => {
  //   const hero_ref = refs.hero_ref.current;
  //   const why_us_ref = refs.why_us_ref.current;
  //   const group_ref = refs.group_ref.current;
  //   const author_reviews_ref = refs.authors_reviews_ref.current;

  //   const hero_observer = new IntersectionObserver(
  //     (entries) => {
  //       const [entry] = entries;
  //       if (!entry.isIntersecting && why_us_ref) {
  //         console.log(`[hero] dark section is intersecting`);
  //         why_us_observer.observe(why_us_ref);
  //       } else if (entry.isIntersecting && why_us_ref) {
  //         console.log(`[hero] bright section is intersecting`);
  //         why_us_observer.unobserve(why_us_ref);
  //       }
  //     },
  //     {
  //       root: null,
  //       threshold: 0.05,
  //     }
  //   );

  //   const why_us_observer = new IntersectionObserver(
  //     (entries) => {
  //       const [entry] = entries;
  //       if (entry.isIntersecting && group_ref) {
  //         console.log(`dark section is intersecting`);
  //         test_observer.unobserve(group_ref);
  //       } else if (!entry.isIntersecting && group_ref) {
  //         console.log(`bright section is intersecting`);
  //         test_observer.observe(group_ref);
  //       }
  //     },
  //     {
  //       root: null,
  //       threshold: 0.05,
  //     }
  //   );

  //   const test_observer = new IntersectionObserver(
  //     (entries) => {
  //       const [entry] = entries;
  //       if (!entry.isIntersecting && author_reviews_ref) {
  //         console.log(`[group] dark section is intersecting`);
  //         author_reviews_observer.observe(author_reviews_ref);
  //       } else if (entry.isIntersecting && author_reviews_ref) {
  //         console.log(`[group] bright section is intersecting`);
  //         author_reviews_observer.unobserve(author_reviews_ref);
  //       }
  //     },
  //     {
  //       root: null,
  //       threshold: 0.05,
  //     }
  //   );

  //   const author_reviews_observer = new IntersectionObserver((entries) => {
  //     const [entry] = entries;
  //     if (entry.isIntersecting) {
  //       console.log(`[author] dark section is intersecting`);
  //     } else {
  //       console.log(`[author bright section is intersecting]`);
  //     }
  //   });

  //   if (hero_ref) {
  //     hero_observer.observe(hero_ref);
  //   }

  //   return () => {
  //     hero_observer.disconnect();
  //     why_us_observer.disconnect();
  //   };
  // }, []);

  return (
    <>
      <div
        ref={refs.hero_ref}
        id="hero"
      >
        <Hero scroll_to_why_us={scroll_to_why_us} />
      </div>
      <div
        ref={refs.why_us_ref}
        id="why_us"
      >
        <WhyUs />
      </div>
      <div
        ref={refs.group_ref}
        id="group"
      >
        <div
          ref={refs.products_ref}
          id="products"
        >
          <Products />
        </div>
        <div
          ref={refs.services_ref}
          id="services"
        >
          <Services />
        </div>
        <div
          ref={refs.join_us_ref}
          id="join_us"
        >
          <JoinUs />
        </div>
      </div>
      <div
        ref={refs.authors_reviews_ref}
        id="authors_reviews"
      >
        <AuthorsReviews />
      </div>
    </>
  );
};

export default HomePage;
