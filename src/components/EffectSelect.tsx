import type { PhotoMode, Effect } from '../types';

interface EffectSelectProps {
  mode: PhotoMode;
  selectedEffect: Effect;
  onSelect: (effect: Effect) => void;
  onConfirm: () => void;
  onBack: () => void;
  loading: boolean;
}

function EffectSelect({
  mode,
  selectedEffect,
  onSelect,
  onConfirm,
  onBack,
  loading,
}: EffectSelectProps) {
  return (
    <div className="effect-select">
      <div className="flex justify-between items-center mb-4">
        <h2 className="heading-2">选择 {mode.name} 效果</h2>
        <button className="btn btn-secondary" onClick={onBack}>
          返回
        </button>
      </div>

      <div className="grid grid-cols-3">
        {mode.effects.map((effect) => (
          <div
            key={effect.id}
            className="card effect-card"
            onClick={() => onSelect(effect)}
            style={{
              cursor: 'pointer',
              textAlign: 'center',
              border: selectedEffect.id === effect.id ? '3px solid var(--color-primary)' : '3px solid transparent',
              transition: 'all 0.2s',
            }}
          >
            <div
              style={{
                width: '100%',
                height: '150px',
                backgroundColor: 'var(--color-border)',
                borderRadius: 'var(--radius-md)',
                marginBottom: '0.5rem',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontSize: '3rem',
              }}
            >
              🖼️
            </div>
            <h3 style={{ fontSize: '1rem', fontWeight: 600 }}>{effect.name}</h3>
            <p className="text-light" style={{ fontSize: '0.875rem' }}>
              下载: ¥{(effect.price_download / 100).toFixed(2)}
            </p>
          </div>
        ))}
      </div>

      <div className="text-center mt-4">
        <button
          className="btn btn-primary btn-lg"
          onClick={onConfirm}
          disabled={loading}
        >
          {loading ? '处理中...' : '开始拍照'}
        </button>
      </div>
    </div>
  );
}

export default EffectSelect;
