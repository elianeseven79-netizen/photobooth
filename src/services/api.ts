import { invoke } from '@tauri-apps/api/core';
import type { PhotoMode, PhotoSession, Order } from '../types';

export const api = {
  // Mode operations
  async getModes(): Promise<PhotoMode[]> {
    console.log('[API] getModes called');
    return invoke<PhotoMode[]>('get_modes');
  },

  async getMode(modeId: string): Promise<PhotoMode | null> {
    console.log('[API] getMode called:', modeId);
    return invoke<PhotoMode | null>('get_mode', { modeId });
  },

  // Session operations
  async createSession(modeId: string, effectId: string): Promise<PhotoSession> {
    console.log('[API] createSession called:', { modeId, effectId });
    return invoke<PhotoSession>('create_session', { modeId, effectId });
  },

  async getSession(sessionId: string): Promise<PhotoSession | null> {
    console.log('[API] getSession called:', sessionId);
    return invoke<PhotoSession | null>('get_session', { sessionId });
  },

  // Photo operations
  async generatePhoto(sessionId: string, photoBase64: string, styleId?: string): Promise<PhotoSession> {
    console.log('[API] generatePhoto called, sessionId:', sessionId, 'photo length:', photoBase64.length, 'styleId:', styleId);
    const result = await invoke<PhotoSession>('generate_photo', { sessionId, photoBase64, styleId });
    console.log('[API] generatePhoto completed, generated photo length:', result.generated_photo?.length || 0);
    return result;
  },

  async saveOriginalPhoto(sessionId: string, photoBase64: string): Promise<void> {
    console.log('[API] saveOriginalPhoto called, sessionId:', sessionId, 'photo length:', photoBase64.length);
    return invoke<void>('save_original_photo', { sessionId, photoBase64 });
  },

  async saveGeneratedPhoto(sessionId: string, photoBase64: string): Promise<void> {
    return invoke<void>('save_generated_photo', { sessionId, photoBase64 });
  },

  // Order operations
  async createOrder(sessionId: string, orderType: string, amount: number): Promise<Order> {
    return invoke<Order>('create_order', { sessionId, orderType, amount });
  },

  async getOrder(orderId: string): Promise<Order | null> {
    return invoke<Order | null>('get_order', { orderId });
  },

  // Payment operations
  async createPayment(sessionId: string, orderType: string, amount: number): Promise<[string, string]> {
    console.log('[API] createPayment called:', { sessionId, orderType, amount });
    return invoke<[string, string]>('create_payment', { sessionId, orderType, amount });
  },

  async queryPayment(orderId: string): Promise<string> {
    return invoke<string>('query_payment', { orderId });
  },
};
