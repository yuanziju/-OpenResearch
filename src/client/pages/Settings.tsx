import { useEffect, useState } from 'react'

interface SettingsState {
  openaiKey: string
  anthropicKey: string
  theme: string
}

const STORAGE_KEY = 'open-research.settings'

const defaultSettings: SettingsState = {
  openaiKey: '',
  anthropicKey: '',
  theme: 'dark',
}

function loadSettings(): SettingsState {
  if (typeof window === 'undefined') return defaultSettings
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY)
    if (!raw) return defaultSettings
    const parsed = JSON.parse(raw) as Partial<SettingsState>
    return { ...defaultSettings, ...parsed }
  } catch {
    return defaultSettings
  }
}

function saveSettings(settings: SettingsState): void {
  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(settings))
  } catch {
    // ignore quota / serialization errors
  }
}

const headingStyle: React.CSSProperties = {
  fontSize: '28px',
  color: '#e0e0e0',
  margin: '0 0 24px 0',
}

const sectionStyle: React.CSSProperties = {
  background: '#16213e',
  padding: '20px',
  borderRadius: '8px',
  marginBottom: '16px',
  boxSizing: 'border-box',
}

const sectionTitleStyle: React.CSSProperties = {
  fontSize: '18px',
  color: '#e0e0e0',
  margin: '0 0 16px 0',
}

const fieldGroupStyle: React.CSSProperties = {
  display: 'flex',
  flexDirection: 'column',
  gap: '4px',
  marginBottom: '12px',
}

const labelStyle: React.CSSProperties = {
  fontSize: '13px',
  color: '#a0a0b0',
}

const inputStyle: React.CSSProperties = {
  padding: '8px 10px',
  fontSize: '14px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: '#1a1a2e',
  color: '#e0e0e0',
  boxSizing: 'border-box',
}

const textStyle: React.CSSProperties = {
  fontSize: '14px',
  color: '#e0e0e0',
  fontFamily: 'monospace',
  padding: '8px 10px',
  background: '#1a1a2e',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
}

const saveButtonStyle: React.CSSProperties = {
  padding: '10px 24px',
  fontSize: '14px',
  borderRadius: '6px',
  border: 'none',
  background: '#e94560',
  color: '#ffffff',
  cursor: 'pointer',
  fontWeight: 'bold',
  marginTop: '8px',
}

const testButtonStyle: React.CSSProperties = {
  padding: '6px 14px',
  fontSize: '13px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: 'transparent',
  color: '#a0a0b0',
  cursor: 'pointer',
}

const messageStyle: React.CSSProperties = {
  padding: '8px 12px',
  borderRadius: '6px',
  fontSize: '13px',
  marginTop: '8px',
}

function Settings() {
  const [settings, setSettings] = useState<SettingsState>(defaultSettings)
  const [status, setStatus] = useState<{ kind: 'saved' | 'info' | null; text: string }>({
    kind: null,
    text: '',
  })

  // Load persisted settings on mount.
  useEffect(() => {
    setSettings(loadSettings())
  }, [])

  const update = <K extends keyof SettingsState>(key: K, value: SettingsState[K]) => {
    setSettings((prev) => ({ ...prev, [key]: value }))
    setStatus({ kind: null, text: '' })
  }

  const handleSave = () => {
    saveSettings(settings)
    setStatus({ kind: 'saved', text: 'Settings saved.' })
  }

  const handleTest = () => {
    setStatus({
      kind: 'info',
      text: 'Connection test is not available in this build. Your API keys have been saved locally.',
    })
  }

  return (
    <div>
      <h1 style={headingStyle}>Settings</h1>

      <div style={sectionStyle}>
        <h2 style={sectionTitleStyle}>API Keys</h2>
        <div style={fieldGroupStyle}>
          <label style={labelStyle}>OpenAI API Key</label>
          <input
            type="password"
            placeholder="sk-..."
            value={settings.openaiKey}
            onChange={(e) => update('openaiKey', e.target.value)}
            style={inputStyle}
          />
        </div>
        <div style={fieldGroupStyle}>
          <label style={labelStyle}>Anthropic API Key</label>
          <input
            type="password"
            placeholder="sk-ant-..."
            value={settings.anthropicKey}
            onChange={(e) => update('anthropicKey', e.target.value)}
            style={inputStyle}
          />
        </div>
        <button type="button" style={testButtonStyle} onClick={handleTest}>
          Test connection
        </button>
      </div>

      <div style={sectionStyle}>
        <h2 style={sectionTitleStyle}>Database</h2>
        <div style={textStyle}>~/.open-research/open-research.db</div>
      </div>

      <div style={sectionStyle}>
        <h2 style={sectionTitleStyle}>Appearance</h2>
        <div style={fieldGroupStyle}>
          <label style={labelStyle}>Theme</label>
          <select
            style={inputStyle}
            value={settings.theme}
            onChange={(e) => update('theme', e.target.value)}
          >
            <option value="dark">Dark</option>
            <option value="light">Light</option>
            <option value="system">System</option>
          </select>
        </div>
      </div>

      <button type="button" style={saveButtonStyle} onClick={handleSave}>
        Save
      </button>
      {status.kind && (
        <div
          style={{
            ...messageStyle,
            color: status.kind === 'saved' ? '#7ee787' : '#a0a0b0',
            background: status.kind === 'saved' ? '#0f2a1a' : '#1a1a2e',
          }}
        >
          {status.text}
        </div>
      )}
    </div>
  )
}

export default Settings
