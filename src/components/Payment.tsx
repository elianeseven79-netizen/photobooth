import { useState } from 'react';
import { QRCodeSVG } from 'qrcode.react';
import type { PhotoSession, Order } from '../types';
import { api } from '../services/api';

interface PaymentProps {
  session: PhotoSession;
  onSuccess: (order: Order) => void;
  onBack: () => void;
}

const DOWNLOAD_PRICE = 300; // 3元

function Payment({ session, onSuccess, onBack }: PaymentProps) {
  const [qrCode, setQrCode] = useState<string>('');
  const [, setOrderId] = useState<string>('');
  const [status, setStatus] = useState<'pending' | 'paid' | 'checking'>('pending');
  const [loading, setLoading] = useState(false);

  const createPayment = async () => {
    setLoading(true);
    try {
      const [newOrderId, qr] = await api.createPayment(session.id, 'download', DOWNLOAD_PRICE);
      setQrCode(qr);
      setOrderId(newOrderId);
      setStatus('pending');

      // Poll for payment status
      pollPaymentStatus(newOrderId);
    } catch (error) {
      console.error('Failed to create payment:', error);
    } finally {
      setLoading(false);
    }
  };

  const pollPaymentStatus = async (orderId: string) => {
    const checkStatus = async () => {
      try {
        const result = await api.queryPayment(orderId);
        if (result === 'SUCCESS') {
          setStatus('paid');
          const order = await api.getOrder(orderId);
          if (order) {
            onSuccess(order);
          }
        } else {
          // Continue polling
          setTimeout(checkStatus, 2000);
        }
      } catch (error) {
        console.error('Failed to query payment:', error);
        setTimeout(checkStatus, 2000);
      }
    };

    checkStatus();
  };

  return (
    <div className="payment">
      <div className="flex justify-between items-center mb-4">
        <h2 className="heading-2">支付</h2>
        <button className="btn btn-secondary" onClick={onBack}>
          返回
        </button>
      </div>

      <div className="card" style={{ maxWidth: '500px', margin: '0 auto', textAlign: 'center' }}>
        {!qrCode ? (
          <>
            <h3 className="mb-4">下载照片</h3>
            <p className="text-light mb-4">价格: ¥{(DOWNLOAD_PRICE / 100).toFixed(2)}</p>
            <button
              className="btn btn-primary btn-lg"
              onClick={createPayment}
              disabled={loading}
            >
              {loading ? '生成中...' : '生成支付二维码'}
            </button>
          </>
        ) : (
          <>
            <h3 className="mb-4">扫码支付</h3>
            <div style={{ padding: '1rem', backgroundColor: 'white', display: 'inline-block', borderRadius: 'var(--radius-md)' }}>
              <QRCodeSVG value={qrCode} size={200} />
            </div>
            <p className="mt-4 text-light">请使用微信扫描二维码完成支付</p>

            {status === 'paid' && (
              <div className="mt-4" style={{ color: 'var(--color-success)' }}>
                支付成功！
              </div>
            )}

            {status === 'pending' && (
              <div className="mt-4">
                <div className="spinner" style={{ margin: '0 auto' }}></div>
                <p className="text-light mt-4">等待支付中...</p>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}

export default Payment;
