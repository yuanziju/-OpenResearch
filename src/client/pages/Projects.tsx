import { useEffect, useState } from 'react'
import { useProjectsStore } from '@/client/state'

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
  marginBottom: '16px',
}

const formStyle: React.CSSProperties = {
  background: '#16213e',
  padding: '16px',
  borderRadius: '8px',
  marginBottom: '24px',
  display: 'flex',
  flexDirection: 'column',
  gap: '8px',
  maxWidth: '480px',
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

const secondaryButtonStyle: React.CSSProperties = {
  padding: '6px 12px',
  fontSize: '12px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: 'transparent',
  color: '#a0a0b0',
  cursor: 'pointer',
}

const saveButtonStyle: React.CSSProperties = {
  padding: '10px 20px',
  fontSize: '14px',
  borderRadius: '6px',
  border: 'none',
  background: '#e94560',
  color: '#ffffff',
  cursor: 'pointer',
  fontWeight: 'bold',
}

const cancelButtonStyle: React.CSSProperties = {
  padding: '10px 20px',
  fontSize: '14px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: 'transparent',
  color: '#a0a0b0',
  cursor: 'pointer',
}

const listStyle: React.CSSProperties = {
  display: 'flex',
  flexDirection: 'column',
  gap: '8px',
}

const projectCardStyle: React.CSSProperties = {
  background: '#16213e',
  padding: '16px',
  borderRadius: '8px',
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'flex-start',
  gap: '12px',
}

const projectNameStyle: React.CSSProperties = {
  fontSize: '16px',
  fontWeight: 'bold',
  color: '#e0e0e0',
  margin: '0 0 4px 0',
}

const projectMetaStyle: React.CSSProperties = {
  fontSize: '13px',
  color: '#a0a0b0',
  margin: 0,
}

const emptyStateStyle: React.CSSProperties = {
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  minHeight: '300px',
  color: '#a0a0b0',
  fontSize: '16px',
}

const messageStyle: React.CSSProperties = {
  padding: '10px 14px',
  borderRadius: '6px',
  marginBottom: '16px',
  fontSize: '14px',
}

function formatDate(iso: string): string {
  if (!iso) return ''
  try {
    return new Date(iso).toLocaleDateString()
  } catch {
    return iso
  }
}

function Projects() {
  const projects = useProjectsStore((s) => s.projects)
  const loading = useProjectsStore((s) => s.loading)
  const error = useProjectsStore((s) => s.error)
  const fetchProjects = useProjectsStore((s) => s.fetchProjects)
  const createProject = useProjectsStore((s) => s.createProject)
  const deleteProject = useProjectsStore((s) => s.deleteProject)

  const [showForm, setShowForm] = useState(false)
  const [name, setName] = useState('')
  const [description, setDescription] = useState('')

  useEffect(() => {
    fetchProjects()
  }, [fetchProjects])

  const handleCreate = async () => {
    if (!name.trim()) return
    const created = await createProject(name.trim(), description.trim())
    if (created) {
      setName('')
      setDescription('')
      setShowForm(false)
    }
  }

  const handleDelete = async (id: string) => {
    if (window.confirm('Delete this project?')) {
      await deleteProject(id)
    }
  }

  return (
    <div>
      <h1 style={headingStyle}>Projects</h1>
      <button
        type="button"
        style={newProjectButtonStyle}
        onClick={() => setShowForm((v) => !v)}
      >
        {showForm ? 'Cancel' : 'New Project'}
      </button>
      {showForm && (
        <div style={formStyle}>
          <input
            type="text"
            placeholder="Project name"
            value={name}
            onChange={(e) => setName(e.target.value)}
            style={inputStyle}
            autoFocus
          />
          <textarea
            placeholder="Description (optional)"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            style={{ ...inputStyle, minHeight: '80px', resize: 'vertical' }}
          />
          <div style={{ display: 'flex', gap: '8px' }}>
            <button type="button" style={saveButtonStyle} onClick={handleCreate} disabled={!name.trim()}>
              Create
            </button>
            <button
              type="button"
              style={cancelButtonStyle}
              onClick={() => {
                setShowForm(false)
                setName('')
                setDescription('')
              }}
            >
              Cancel
            </button>
          </div>
        </div>
      )}

      {error && (
        <div style={{ ...messageStyle, color: '#e94560', background: '#2a1a2a' }}>{error}</div>
      )}

      {loading && projects.length === 0 ? (
        <div style={emptyStateStyle}>Loading projects…</div>
      ) : projects.length === 0 ? (
        <div style={emptyStateStyle}>
          No projects yet. Create a project to organize your research.
        </div>
      ) : (
        <div style={listStyle}>
          {projects.map((project) => (
            <div key={project.id} style={projectCardStyle}>
              <div style={{ flex: 1, minWidth: 0 }}>
                <h3 style={projectNameStyle}>{project.name}</h3>
                {project.description && (
                  <p style={projectMetaStyle}>{project.description}</p>
                )}
                <p style={{ ...projectMetaStyle, marginTop: '4px' }}>
                  Created {formatDate(project.createdAt)}
                  {project.members.length > 0 ? ` · ${project.members.length} member${project.members.length === 1 ? '' : 's'}` : ''}
                </p>
              </div>
              <button
                type="button"
                style={secondaryButtonStyle}
                onClick={() => handleDelete(project.id)}
              >
                Delete
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}

export default Projects
