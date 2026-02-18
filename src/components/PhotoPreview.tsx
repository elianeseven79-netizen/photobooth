import { useState, useEffect } from 'react';
import type { PhotoSession } from '../types';
import { api } from '../services/api';

interface PhotoPreviewProps {
  session: PhotoSession;
  onRegenerate: (photoBase64: string, styleId?: string) => void;
  onConfirm: () => void;
  onChangeEffect: () => void;
  onChangeStyle: () => void;
  onBack: () => void;
  loading: boolean;
}

function PhotoPreview({
  session,
  onRegenerate,
  onConfirm,
  onChangeEffect,
  onChangeStyle,
  onBack,
  loading,
}: PhotoPreviewProps) {
  // Use session data directly first, then refresh from API
  const [photo, setPhoto] = useState<string | null>(session.generated_photo || null);
  const [originalPhoto, setOriginalPhoto] = useState<string | null>(session.original_photo || null);

  useEffect(() => {
    // Refresh session data from API in case it was updated
    loadSession();
  }, [session.id]);

  const loadSession = async () => {
    try {
      const data = await api.getSession(session.id);
      if (data?.generated_photo) {
        setPhoto(data.generated_photo);
      }
      if (data?.original_photo) {
        setOriginalPhoto(data.original_photo);
      }
    } catch (error) {
      console.error('Failed to load session:', error);
    }
  };

  const handleRegenerate = async () => {
    if (!originalPhoto) return;
    // Pass the style_id if available
    onRegenerate(originalPhoto, session.style_id);
  };

  return (
    <div className="photo-preview">
      <div className="flex justify-between items-center mb-4">
        <h2 className="heading-2">预览效果</h2>
        <button className="btn btn-secondary" onClick={onBack}>
          返回
        </button>
      </div>

      <div className="grid grid-cols-2" style={{ gap: '2rem', maxWidth: '900px', margin: '0 auto' }}>
        <div className="card">
          <h3 className="text-center mb-4">原图</h3>
          {originalPhoto ? (
            <img
              src={`data:image/jpeg;base64,${originalPhoto}`}
              alt="Original"
              style={{ width: '100%', borderRadius: 'var(--radius-md)' }}
            />
          ) : (
            <div
              style={{
                width: '100%',
                height: '300px',
                backgroundColor: 'var(--color-border)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                borderRadius: 'var(--radius-md)',
              }}
            >
              <span className="text-light">暂无原图</span>
            </div>
          )}
        </div>

        <div className="card">
          <h3 className="text-center mb-4">AI 生成</h3>
          {loading ? (
            <div style={{ textAlign: 'center', padding: '2rem' }}>
              <div className="spinner" style={{ margin: '0 auto 1rem' }}></div>
              <p>AI 正在生成中，请稍候...</p>
            </div>
          ) : photo ? (
            <img
              src={`data:image/jpeg;base64,${photo}`}
              alt="Generated"
              style={{ width: '100%', borderRadius: 'var(--radius-md)' }}
            />
          ) : (
            <div
              style={{
                width: '100%',
                height: '300px',
                backgroundColor: 'var(--color-border)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                borderRadius: 'var(--radius-md)',
              }}
            >
              <span className="text-light">等待生成...</span>
            </div>
          )}
        </div>
      </div>

      <div className="text-center mt-4" style={{ display: 'flex', gap: '1rem', justifyContent: 'center', flexWrap: 'wrap' }}>
        <button className="btn btn-secondary" onClick={onChangeEffect}>
          更换效果
        </button>
        <button className="btn btn-secondary" onClick={onChangeStyle}>
          更换风格
        </button>
        <button className="btn btn-secondary" onClick={handleRegenerate} disabled={loading || !originalPhoto}>
          重新生成
        </button>
        <button className="btn btn-primary" onClick={onConfirm} disabled={loading}>
          确认使用
        </button>
      </div>
    </div>
  );
}

export default PhotoPreview;
