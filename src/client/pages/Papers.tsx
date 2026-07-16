// TODO: depends on agent-04 implementing API client

const headingStyle: React.CSSProperties = {
  fontSize: '28px',
  color: '#e0e0e0',
  margin: '0 0 24px 0',
}

const searchInputStyle: React.CSSProperties = {
  width: '100%',
  maxWidth: '480px',
  padding: '10px 14px',
  fontSize: '14px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: '#16213e',
  color: '#e0e0e0',
  boxSizing: 'border-box',
  marginBottom: '32px',
}

const emptyStateStyle: React.CSSProperties = {
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  minHeight: '300px',
  color: '#a0a0b0',
  fontSize: '16px',
}

function Papers() {
  return (
    <div>
      <h1 style={headingStyle}>Papers</h1>
      <input
        type="text"
        placeholder="Search papers..."
        style={searchInputStyle}
      />
      <div style={emptyStateStyle}>
        No papers yet. Import a paper to get started.
      </div>
    </div>
  )
}

export default Papers
