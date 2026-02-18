import { useState, useRef, useEffect } from 'react';
import { api } from '../services/api';
import type { PhotoSession } from '../types';

interface CameraProps {
  sessionId: string;
  onCapture: (session?: PhotoSession) => void;
  onBack: () => void;
}

// 模拟拍照 - 使用预设图片
const loadMockPhoto = async (): Promise<string> => {
  try {
    const response = await fetch('/mock-photo.jpg');
    const blob = await response.blob();
    return new Promise((resolve) => {
      const reader = new FileReader();
      reader.onloadend = () => {
        const base64 = reader.result as string;
        resolve(base64.split(',')[1]);
      };
      reader.readAsDataURL(blob);
    });
  } catch (error) {
    console.error('Failed to load mock photo:', error);
    // 如果加载失败，生成一个简单的替代图片
    const canvas = document.createElement('canvas');
    canvas.width = 640;
    canvas.height = 480;
    const ctx = canvas.getContext('2d');
    if (!ctx) return '';
    ctx.fillStyle = '#667eea';
    ctx.fillRect(0, 0, 640, 480);
    ctx.fillStyle = 'white';
    ctx.font = 'bold 48px sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('模拟照片', 320, 240);
    return canvas.toDataURL('image/jpeg', 0.8).split(',')[1];
  }
};

function Camera({ sessionId, onCapture, onBack }: CameraProps) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [countdown, setCountdown] = useState<number | null>(null);
  const [capturedPhoto, setCapturedPhoto] = useState<string | null>(null);
  const [stream, setStream] = useState<MediaStream | null>(null);
  const [cameraError, setCameraError] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    startCamera();
    return () => {
      if (stream) {
        stream.getTracks().forEach(track => track.stop());
      }
    };
  }, []);

  const startCamera = async () => {
    try {
      const mediaStream = await navigator.mediaDevices.getUserMedia({
        video: { width: 1280, height: 720, facingMode: 'user' },
      });
      setStream(mediaStream);
      if (videoRef.current) {
        videoRef.current.srcObject = mediaStream;
      }
    } catch (error) {
      console.error('Failed to access camera:', error);
      setCameraError(true);
    }
  };

  const startCountdown = () => {
    setCountdown(3);
    const timer = setInterval(() => {
      setCountdown((prev) => {
        if (prev === 1) {
          clearInterval(timer);
          takePhoto();
          return null;
        }
        return (prev || 1) - 1;
      });
    }, 1000);
  };

  const takePhoto = () => {
    if (!videoRef.current || !canvasRef.current) return;

    const video = videoRef.current;
    const canvas = canvasRef.current;
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.drawImage(video, 0, 0);
    const photoBase64 = canvas.toDataURL('image/jpeg', 0.8).split(',')[1];
    setCapturedPhoto(photoBase64);

    // Stop camera stream
    if (stream) {
      stream.getTracks().forEach(track => track.stop());
    }
  };

  // 模拟拍照 - 用于没有摄像头的测试
  const simulatePhoto = () => {
    setCountdown(3);
    setTimeout(async () => {
      const mockPhoto = await loadMockPhoto();
      setCapturedPhoto(mockPhoto);
      setCountdown(null);
    }, 3000);
  };

  const handleConfirm = async () => {
    if (!capturedPhoto) return;

    console.log('[Camera] Starting photo generation process...');
    console.log('[Camera] Session ID:', sessionId);
    console.log('[Camera] Photo base64 length:', capturedPhoto.length);
    setIsProcessing(true);
    setError(null);

    // 设置超时 - 60秒后自动进入下一步（方便测试）
    const timeoutId = setTimeout(() => {
      console.warn('[Camera] Generation timeout, proceeding anyway...');
      setIsProcessing(false);
      onCapture();
    }, 60000);

    try {
      console.log('[Camera] Step 1: Saving original photo...');
      await api.saveOriginalPhoto(sessionId, capturedPhoto);
      console.log('[Camera] Step 1 complete: Original photo saved');

      console.log('[Camera] Step 2: Calling MiniMax API to generate photo...');
      const updatedSession = await api.generatePhoto(sessionId, capturedPhoto);
      console.log('[Camera] Step 2 complete: Generated photo received');
      console.log('[Camera] Generated photo length:', updatedSession.generated_photo?.length || 0);

      clearTimeout(timeoutId);
      // Pass the updated session with generated photo
      onCapture(updatedSession);
    } catch (err) {
      clearTimeout(timeoutId);
      console.error('[Camera] Error during photo generation:', err);
      setError(err instanceof Error ? err.message : String(err));
      // 即使失败也进入下一步，方便测试
      console.log('[Camera] Proceeding to preview despite error...');
      onCapture();
    } finally {
      setIsProcessing(false);
    }
  };

  const handleRetake = () => {
    setCapturedPhoto(null);
    if (!cameraError) {
      startCamera();
    }
  };

  return (
    <div className="camera">
      <div className="flex justify-between items-center mb-4">
        <h2 className="heading-2">拍照</h2>
        <button className="btn btn-secondary" onClick={onBack}>
          返回
        </button>
      </div>

      <div className="card" style={{ maxWidth: '800px', margin: '0 auto' }}>
        <div style={{ position: 'relative', backgroundColor: '#000', borderRadius: 'var(--radius-md)', overflow: 'hidden', minHeight: '400px' }}>
          {countdown && (
            <div
              style={{
                position: 'absolute',
                top: '50%',
                left: '50%',
                transform: 'translate(-50%, -50%)',
                fontSize: '8rem',
                fontWeight: 'bold',
                color: 'white',
                zIndex: 10,
              }}
            >
              {countdown}
            </div>
          )}

          {!capturedPhoto ? (
            cameraError ? (
              <div style={{
                width: '100%',
                height: '400px',
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                justifyContent: 'center',
                color: 'white',
              }}>
                <div style={{ fontSize: '4rem', marginBottom: '1rem' }}>📷</div>
                <p>没有检测到摄像头</p>
                <p style={{ fontSize: '0.875rem', opacity: 0.7 }}>点击下方"模拟拍照"按钮继续</p>
              </div>
            ) : (
              <video
                ref={videoRef}
                autoPlay
                playsInline
                muted
                style={{ width: '100%', display: 'block' }}
              />
            )
          ) : (
            <img
              src={`data:image/jpeg;base64,${capturedPhoto}`}
              alt="Captured"
              style={{ width: '100%', display: 'block' }}
            />
          )}

          {/* Loading overlay when processing AI photo */}
          {isProcessing && (
            <div style={{
              position: 'absolute',
              top: 0,
              left: 0,
              right: 0,
              bottom: 0,
              backgroundColor: 'rgba(26, 26, 46, 0.95)',
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
              color: 'white',
              zIndex: 20,
            }}>
              <div style={{ fontSize: '3rem', marginBottom: '1rem' }}>🎨</div>
              <p style={{ fontSize: '1.25rem', marginBottom: '0.5rem' }}>正在生成 AI 照片...</p>
              <p style={{ fontSize: '0.875rem', opacity: 0.7 }}>请稍候</p>
              {error && (
                <p style={{ fontSize: '0.875rem', color: '#ff6b6b', marginTop: '1rem' }}>
                  错误: {error}
                </p>
              )}
              <div style={{
                width: '200px',
                height: '4px',
                backgroundColor: 'rgba(255,255,255,0.2)',
                borderRadius: '2px',
                marginTop: '1.5rem',
                overflow: 'hidden',
              }}>
                <div style={{
                  width: '50%',
                  height: '100%',
                  backgroundColor: '#667eea',
                  borderRadius: '2px',
                  animation: 'loading 1.5s ease-in-out infinite',
                }} />
              </div>
            </div>
          )}
        </div>

        <canvas ref={canvasRef} style={{ display: 'none' }} />

        <div className="text-center mt-4">
          {!capturedPhoto ? (
            <div style={{ display: 'flex', gap: '1rem', justifyContent: 'center' }}>
              {cameraError ? (
                <button
                  className="btn btn-primary btn-lg"
                  onClick={simulatePhoto}
                  disabled={countdown !== null}
                >
                  {countdown !== null ? '模拟中...' : '模拟拍照 (测试)'}
                </button>
              ) : (
                <button
                  className="btn btn-primary btn-lg"
                  onClick={startCountdown}
                  disabled={countdown !== null}
                >
                  {countdown !== null ? `倒计时 ${countdown}...` : '拍照'}
                </button>
              )}
            </div>
          ) : (
            <div className="flex gap-4 justify-center">
              <button className="btn btn-secondary btn-lg" onClick={handleRetake}>
                重拍
              </button>
              <button className="btn btn-primary btn-lg" onClick={handleConfirm}>
                确认
              </button>
            </div>
          )}
        </div>

        {cameraError && (
          <div className="text-center mt-4" style={{ color: 'var(--color-warning)' }}>
            <p>💡 提示：使用"模拟拍照"可以跳过摄像头继续测试流程</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default Camera;
