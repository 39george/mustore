export interface ToggledLinks {
  products: boolean;
  services: boolean;
  help: boolean;
  about: boolean;
}

export type LinkName = keyof ToggledLinks;

export interface HeroProps {
  scroll_to_why_us: () => void;
}

export type page_names =
  | ""
  | "products/beats"
  | "products/covers"
  | "products/songs"
  | "products/texts"
  | "services"
  | "help"
  | "about";
