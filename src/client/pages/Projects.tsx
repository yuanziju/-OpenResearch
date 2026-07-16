const headingStyle: React.CSSProperties = {
  fontSize: '28px',
  color: '#e0e0e0',
  margin: '0 0 24px 0',
}

const newProjectButtonStyle: React.CSSProperties = {
  padding: '10px 20px',
  fontSize: '14px',
  borderRadius: '6px',
  border: 'none',
  background: '#e94560',
  color: '#ffffff',
  cursor: 'pointer',
  fontWeight: 'bold',
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

function Projects() {
  return (
    <div>
      <h1 style={headingStyle}>Projects</h1>
      <button type="button" style={newProjectButtonStyle}>
        New Project
      </button>
      <div style={emptyStateStyle}>
        No projects yet. Create a project to organize your research.
      </div>
    </div>
  )
}

export default Projects
