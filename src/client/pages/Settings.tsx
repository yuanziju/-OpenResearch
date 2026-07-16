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

function Settings() {
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
            style={inputStyle}
          />
        </div>
        <div style={fieldGroupStyle}>
          <label style={labelStyle}>Anthropic API Key</label>
          <input
            type="password"
            placeholder="sk-ant-..."
            style={inputStyle}
          />
        </div>
      </div>

      <div style={sectionStyle}>
        <h2 style={sectionTitleStyle}>Database</h2>
        <div style={textStyle}>~/.open-research/open-research.db</div>
      </div>

      <div style={sectionStyle}>
        <h2 style={sectionTitleStyle}>Appearance</h2>
        <div style={fieldGroupStyle}>
          <label style={labelStyle}>Theme</label>
          <select style={inputStyle} defaultValue="dark">
            <option value="dark">Dark</option>
            <option value="light">Light</option>
            <option value="system">System</option>
          </select>
        </div>
      </div>

      <button type="button" style={saveButtonStyle}>
        Save
      </button>
    </div>
  )
}

export default Settings
