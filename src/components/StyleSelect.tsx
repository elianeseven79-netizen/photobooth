interface StyleSelectProps {
  originalPhoto: string;
  onGenerate: (styleId?: string) => void;
  onBack: () => void;
  loading: boolean;
}

function StyleSelect({ originalPhoto, onGenerate, onBack, loading }: StyleSelectProps) {
  const handleGenerate = () => {
    console.log('[StyleSelect] Generating AI photo');
    onGenerate(undefined);
  };

  return (
    <div className="style-select">
      <div className="flex justify-between items-center mb-4">
        <h2 className="heading-2">生成AI照片</h2>
        <button className="btn btn-secondary" onClick={onBack}>
          返回
        </button>
      </div>

      {/* Preview section */}
      <div style={{ maxWidth: '600px', margin: '2rem auto' }}>
        <h3 className="text-center mb-2">原图预览</h3>
        <div className="card" style={{ position: 'relative', overflow: 'hidden' }}>
          <img
            src={`data:image/jpeg;base64,${originalPhoto}`}
            alt="Original"
            style={{ width: '100%', borderRadius: 'var(--radius-md)' }}
          />
          {loading && (
            <div
              style={{
                position: 'absolute',
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
                backgroundColor: 'rgba(0, 0, 0, 0.6)',
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                justifyContent: 'center',
                borderRadius: 'var(--radius-md)',
              }}
            >
              <div className="spinner" style={{ marginBottom: '1rem' }}></div>
              <p style={{ color: '#fff', marginBottom: '1rem' }}>AI 正在生成中...</p>
              <div
                style={{
                  width: '60%',
                  height: '8px',
                  backgroundColor: 'rgba(255,255,255,0.3)',
                  borderRadius: '4px',
                  overflow: 'hidden',
                }}
              >
                <div
                  style={{
                    width: '30%',
                    height: '100%',
                    backgroundColor: 'var(--color-primary)',
                    borderRadius: '4px',
                    animation: 'loading 1.5s ease-in-out infinite',
                  }}
                />
              </div>
              <style>{`
                @keyframes loading {
                  0% { transform: translateX(-100%); }
                  50% { transform: translateX(200%); }
                  100% { transform: translateX(-100%); }
                }
              `}</style>
            </div>
          )}
        </div>
      </div>

      <div className="text-center mt-4">
        <button
          className="btn btn-primary btn-lg"
          onClick={handleGenerate}
          disabled={loading}
        >
          {loading ? '生成中...' : '生成AI照片'}
        </button>
      </div>
    </div>
  );
}

export default StyleSelect;
