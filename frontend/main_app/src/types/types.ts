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

export interface CarouselItem {
  id?: number;
  created_at?: Date;
  cover_url: string;
  name: string;
  author: string;
  likes: number;
  price: number;
}

export interface CarouselProps {
  carousel_type: "recommended" | "new";
  carousel_items: CarouselItem[];
}

export interface SongItem {
  id?: number;
  created_at?: Date;
  cover_url: string;
  name: string;
  author: string;
  likes: number;
  listenings: number;
  price: number;
}

export interface CheckedItems {
  [key: string]: boolean;
}

export interface UsernameExistence {
  exists: boolean;
}

export interface IServiceItem {
  icon: string;
  title: string;
  description: string;
}

type MessageAuthor = "interlocutor" | "user";

export interface IConversationUnit {
  interlocutor_name: string;
  message: string;
  message_author: MessageAuthor;
  online_status: boolean;
  time_stamp: string;
  unread_messages: number;
}

type Status = "в работе" | "доставлен";
export interface IOrderUnit {
  consumer: string;
  price: string;
  deliver_to: string;
  status: Status;
}

export type ActiveSections =
  | "dashboard"
  | "products"
  | "services"
  | "conversations"
  | "orders"
  | "statistics"
  | "earnings"
  | "settings"
  | "notifications"
  | "help"
  | "none";
