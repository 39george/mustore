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

export type ActiveTabsAccountCreator =
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
  | "";

export interface UsernameAvatar {
  username: string;
  avatar: string;
  is_loading: boolean;
}

export type UserRole = "creator" | "consumer";

export type ProductStatus =
  | "moderation"
  | "denied"
  | "active"
  | "hidden"
  | "sold"
  | null;

export enum MusicKey {
  a_minor = "a_minor",
  a_major = "a_major",
  b_flat_minor = "b_flat_minor",
  b_flat_major = "b_flat_major",
  b_minor = "b_minor",
  b_major = "b_major",
  c_minor = "c_minor",
  c_major = "c_major",
  c_sharp_minor = "c_sharp_minor",
  c_sharp_major = "c_sharp_major",
  d_minor = "d_minor",
  d_major = "d_major",
  e_flat_minor = "e_flat_minor",
  e_flat_major = "e_flat_major",
  e_minor = "e_minor",
  e_major = "e_major",
  f_minor = "f_minor",
  f_major = "f_major",
  f_sharp_minor = "f_sharp_minor",
  f_sharp_major = "f_sharp_major",
  g_minor = "g_minor",
  g_major = "g_major",
  a_flat_minor = "a_flat_minor",
  a_flat_major = "a_flat_major",
}

export interface IProduct {
  duration: string;
  key: string;
  likes_count: string;
  listenings_count: string;
  lyric: string;
  moods: string[];
  music_key: MusicKey;
  name: string;
  price: string;
  primary_genre: string;
  secondary_genre?: string;
  sex: string;
  song_id: string;
  tempo: string;
}

export enum ProductSectionType {
  songs = "Песни",
  covers = "Обложки",
  beats = "Биты",
  texts = "Тексты",
}

export enum TypeDeclension {
  song = "песню",
  cover = "обложку",
  beat = "бит",
  text = "текст",
}
