const headingStyle: React.CSSProperties = {
  fontSize: '28px',
  color: '#e0e0e0',
  margin: '0 0 24px 0',
}

const columnsStyle: React.CSSProperties = {
  display: 'flex',
  gap: '16px',
  minHeight: '400px',
}

const leftColumnStyle: React.CSSProperties = {
  width: '300px',
  background: '#16213e',
  borderRadius: '8px',
  padding: '16px',
  boxSizing: 'border-box',
}

const rightColumnStyle: React.CSSProperties = {
  flex: 1,
  background: '#16213e',
  borderRadius: '8px',
  padding: '16px',
  boxSizing: 'border-box',
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
}

const newNoteButtonStyle: React.CSSProperties = {
  width: '100%',
  padding: '10px',
  fontSize: '14px',
  borderRadius: '6px',
  border: 'none',
  background: '#e94560',
  color: '#ffffff',
  cursor: 'pointer',
  fontWeight: 'bold',
  marginBottom: '16px',
}

const emptyStateStyle: React.CSSProperties = {
  color: '#a0a0b0',
  fontSize: '16px',
}

function Notes() {
  return (
    <div>
      <h1 style={headingStyle}>Notes</h1>
      <div style={columnsStyle}>
        <div style={leftColumnStyle}>
          <button type="button" style={newNoteButtonStyle}>
            New Note
          </button>
        </div>
        <div style={rightColumnStyle}>
          <span style={emptyStateStyle}>Select or create a note</span>
        </div>
      </div>
    </div>
  )
}

export default Notes
