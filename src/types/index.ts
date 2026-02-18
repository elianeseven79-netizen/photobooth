export interface PhotoMode {
  id: string;
  name: string;
  description: string;
  icon: string;
  effects: Effect[];
}

export interface Effect {
  id: string;
  mode_id: string;
  name: string;
  prompt: string;
  thumbnail: string;
  price_download: number;
  price_print: number;
}

export interface PhotoSession {
  id: string;
  mode_id: string;
  effect_id: string;
  original_photo?: string;
  generated_photo?: string;
  status: SessionStatus;
  created_at: number;
  updated_at: number;
}

export type SessionStatus =
  | 'selecting_mode'
  | 'selecting_effect'
  | 'capturing'
  | 'processing'
  | 'previewing'
  | 'completed';

export interface Order {
  id: string;
  session_id: string;
  order_type: OrderType;
  amount: number;
  status: OrderStatus;
  wechat_order_id?: string;
  payment_time?: number;
  created_at: number;
}

export type OrderType = 'download' | 'print';
export type OrderStatus = 'pending' | 'paid' | 'cancelled' | 'refunded';
