import { useState } from 'react';
import './styles/global.css';
import type { PhotoMode, Effect, PhotoSession, Order } from './types';
import { api } from './services/api';
import ModeSelect from './components/ModeSelect';
import EffectSelect from './components/EffectSelect';
import Camera from './components/Camera';
import StyleSelect from './components/StyleSelect';
import PhotoPreview from './components/PhotoPreview';
import Payment from './components/Payment';
import OrderList from './components/OrderList';

type Step = 'home' | 'selectMode' | 'selectEffect' | 'capture' | 'selectStyle' | 'preview' | 'payment' | 'download';

function App() {
  const [step, setStep] = useState<Step>('home');
  const [modes, setModes] = useState<PhotoMode[]>([]);
  const [selectedMode, setSelectedMode] = useState<PhotoMode | null>(null);
  const [selectedEffect, setSelectedEffect] = useState<Effect | null>(null);
  const [session, setSession] = useState<PhotoSession | null>(null);
  const [capturedPhoto, setCapturedPhoto] = useState<string | null>(null);
  const [, setOrder] = useState<Order | null>(null);
  const [loading, setLoading] = useState(false);

  const loadModes = async () => {
    setLoading(true);
    try {
      const data = await api.getModes();
      setModes(data);
      setStep('selectMode');
    } catch (error) {
      console.error('Failed to load modes:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleModeSelect = (mode: PhotoMode) => {
    setSelectedMode(mode);
    if (mode.effects.length > 0) {
      setSelectedEffect(mode.effects[0]);
    }
    setStep('selectEffect');
  };

  const handleEffectSelect = (effect: Effect) => {
    setSelectedEffect(effect);
  };

  const handleStartCapture = async () => {
    if (!selectedMode || !selectedEffect) return;

    setLoading(true);
    try {
      const newSession = await api.createSession(selectedMode.id, selectedEffect.id);
      setSession(newSession);
      setStep('capture');
    } catch (error) {
      console.error('Failed to create session:', error);
    } finally {
      setLoading(false);
    }
  };

  const handlePhotoCapture = (updatedSession?: PhotoSession) => {
    if (updatedSession?.original_photo) {
      setCapturedPhoto(updatedSession.original_photo);
      setSession(updatedSession);
    }
    // Go to style selection instead of preview
    setStep('selectStyle');
  };

  const handleStyleGenerate = async (styleId?: string) => {
    if (!session || !capturedPhoto || loading) return;

    console.log('[App] handleStyleGenerate called, session.id:', session.id);
    setLoading(true);

    // Prevent multiple clicks
    const currentSessionId = session.id;
    const currentPhoto = capturedPhoto;

    try {
      console.log('[App] Calling API generatePhoto...');
      const updatedSession = await api.generatePhoto(currentSessionId, currentPhoto, styleId || undefined);
      console.log('[App] API returned, generated_photo length:', updatedSession.generated_photo?.length || 0);

      // Only proceed if we're still on selectStyle step and loading is true
      if (updatedSession.generated_photo) {
        setSession(updatedSession);
        setStep('preview');
      }
    } catch (error) {
      console.error('[App] Failed to generate photo:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleRegenerate = async (photoBase64: string, styleId?: string) => {
    if (!session) return;

    setLoading(true);
    try {
      // Use the same style_id if provided, otherwise generate without style
      const updatedSession = await api.generatePhoto(session.id, photoBase64, styleId || session.style_id);
      setSession(updatedSession);
    } catch (error) {
      console.error('Failed to generate photo:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleConfirmPhoto = () => {
    setStep('payment');
  };

  const handlePaymentSuccess = (newOrder: Order) => {
    setOrder(newOrder);
    setStep('download');
  };

  const handleBackToHome = () => {
    setStep('home');
    setSelectedMode(null);
    setSelectedEffect(null);
    setSession(null);
    setCapturedPhoto(null);
    setOrder(null);
  };

  const renderStep = () => {
    switch (step) {
      case 'home':
        return (
          <div className="home-page text-center">
            <h1 className="heading-1">AI 自助拍照机</h1>
            <p className="text-light mb-4">用AI创造独特的照片风格</p>
            <button className="btn btn-primary btn-lg" onClick={loadModes}>
              开始拍照
            </button>
            <div className="mt-4">
              <button className="btn btn-secondary" onClick={() => setStep('download')}>
                我的订单
              </button>
            </div>
          </div>
        );

      case 'selectMode':
        return (
          <ModeSelect
            modes={modes}
            onSelect={handleModeSelect}
            onBack={handleBackToHome}
          />
        );

      case 'selectEffect':
        return (
          <EffectSelect
            mode={selectedMode!}
            selectedEffect={selectedEffect!}
            onSelect={handleEffectSelect}
            onConfirm={handleStartCapture}
            onBack={() => setStep('selectMode')}
            loading={loading}
          />
        );

      case 'capture':
        return (
          <Camera
            sessionId={session!.id}
            onCapture={handlePhotoCapture}
            onBack={() => setStep('selectEffect')}
          />
        );

      case 'selectStyle':
        return (
          <StyleSelect
            originalPhoto={capturedPhoto || session!.original_photo || ''}
            onGenerate={handleStyleGenerate}
            onBack={() => setStep('capture')}
            loading={loading}
          />
        );

      case 'preview':
        return (
          <PhotoPreview
            session={session!}
            onRegenerate={handleRegenerate}
            onConfirm={handleConfirmPhoto}
            onChangeEffect={() => setStep('selectEffect')}
            onChangeStyle={() => setStep('selectStyle')}
            onBack={() => setStep('selectStyle')}
            loading={loading}
          />
        );

      case 'payment':
        return (
          <Payment
            session={session!}
            onSuccess={handlePaymentSuccess}
            onBack={() => setStep('preview')}
          />
        );

      case 'download':
        return (
          <OrderList
            onBack={handleBackToHome}
            onNewPhoto={handleBackToHome}
          />
        );

      default:
        return null;
    }
  };

  return (
    <div className="app">
      <main className="main-content">
        {renderStep()}
      </main>
    </div>
  );
}

export default App;
