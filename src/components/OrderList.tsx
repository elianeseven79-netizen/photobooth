import { useState, useEffect } from 'react';
import type { Order } from '../types';
import { api } from '../services/api';

interface OrderListProps {
  onBack: () => void;
  onNewPhoto: () => void;
}

function OrderList({ onBack, onNewPhoto }: OrderListProps) {
  const [orders, setOrders] = useState<Order[]>([]);
  const [loading, setLoading] = useState(true);
  const [downloadingOrderId, setDownloadingOrderId] = useState<string | null>(null);

  useEffect(() => {
    loadOrders();
  }, []);

  const loadOrders = async () => {
    setLoading(true);
    try {
      // For demo, get orders from session storage
      const savedOrders = sessionStorage.getItem('orders');
      if (savedOrders) {
        setOrders(JSON.parse(savedOrders));
      }
    } catch (error) {
      console.error('Failed to load orders:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = async (order: Order) => {
    setDownloadingOrderId(order.id);
    try {
      const session = await api.getSession(order.session_id);
      if (session?.generated_photo) {
        // Create download link
        const link = document.createElement('a');
        link.href = `data:image/jpeg;base64,${session.generated_photo}`;
        link.download = `ai-photo-${order.id}.jpg`;
        link.click();
      }
    } catch (error) {
      console.error('Failed to download:', error);
    } finally {
      setDownloadingOrderId(null);
    }
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString('zh-CN');
  };

  const formatPrice = (cents: number) => {
    return `¥${(cents / 100).toFixed(2)}`;
  };

  return (
    <div className="order-list">
      <div className="flex justify-between items-center mb-4">
        <h2 className="heading-2">我的订单</h2>
        <button className="btn btn-secondary" onClick={onBack}>
          返回首页
        </button>
      </div>

      <div className="card">
        {loading ? (
          <div className="loading">
            <div className="spinner"></div>
          </div>
        ) : orders.length === 0 ? (
          <div className="text-center p-4">
            <p className="text-light">暂无订单</p>
            <button className="btn btn-primary mt-4" onClick={onNewPhoto}>
              立即拍照
            </button>
          </div>
        ) : (
          <div>
            {orders.map((order) => (
              <div
                key={order.id}
                style={{
                  padding: '1rem',
                  borderBottom: '1px solid var(--color-border)',
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                }}
              >
                <div>
                  <p style={{ fontWeight: 600 }}>订单号: {order.id.slice(0, 8)}...</p>
                  <p className="text-light" style={{ fontSize: '0.875rem' }}>
                    {formatDate(order.created_at)} | {formatPrice(order.amount)}
                  </p>
                  <p
                    style={{
                      fontSize: '0.875rem',
                      color:
                        order.status === 'paid'
                          ? 'var(--color-success)'
                          : 'var(--color-text-light)',
                    }}
                  >
                    {order.status === 'paid' ? '已支付' : '未支付'}
                  </p>
                </div>
                {order.status === 'paid' && (
                  <button
                    className="btn btn-primary"
                    onClick={() => handleDownload(order)}
                    disabled={downloadingOrderId === order.id}
                  >
                    {downloadingOrderId === order.id ? '下载中...' : '下载'}
                  </button>
                )}
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="text-center mt-4">
        <button className="btn btn-primary btn-lg" onClick={onNewPhoto}>
          继续拍照
        </button>
      </div>
    </div>
  );
}

export default OrderList;
