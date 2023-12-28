export interface ToggledLinks {
  products: boolean;
  services: boolean;
  help: boolean;
  about: boolean;
}

export type LinkName = keyof ToggledLinks;
