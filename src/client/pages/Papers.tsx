import { useEffect, useMemo, useState } from 'react'
import { usePapersStore } from '@/client/state'
import type { Paper } from '@/shared/types'

const headingStyle: React.CSSProperties = {
  fontSize: '28px',
  color: '#e0e0e0',
  margin: '0 0 24px 0',
}

const topRowStyle: React.CSSProperties = {
  display: 'flex',
  gap: '8px',
  marginBottom: '32px',
  flexWrap: 'wrap',
}

const searchInputStyle: React.CSSProperties = {
  flex: 1,
  minWidth: '200px',
  padding: '10px 14px',
  fontSize: '14px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: '#16213e',
  color: '#e0e0e0',
  boxSizing: 'border-box',
}

const addButtonStyle: React.CSSProperties = {
  padding: '10px 20px',
  fontSize: '14px',
  borderRadius: '6px',
  border: 'none',
  background: '#e94560',
  color: '#ffffff',
  cursor: 'pointer',
  fontWeight: 'bold',
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

const listStyle: React.CSSProperties = {
  display: 'flex',
  flexDirection: 'column',
  gap: '8px',
}

const paperCardStyle: React.CSSProperties = {
  background: '#16213e',
  padding: '16px',
  borderRadius: '8px',
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'flex-start',
  gap: '12px',
}

const paperTitleStyle: React.CSSProperties = {
  fontSize: '16px',
  fontWeight: 'bold',
  color: '#e0e0e0',
  margin: '0 0 4px 0',
}

const paperMetaStyle: React.CSSProperties = {
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

function Papers() {
  const papers = usePapersStore((s) => s.papers)
  const loading = usePapersStore((s) => s.loading)
  const error = usePapersStore((s) => s.error)
  const fetchPapers = usePapersStore((s) => s.fetchPapers)
  const createPaper = usePapersStore((s) => s.createPaper)
  const deletePaper = usePapersStore((s) => s.deletePaper)

  const [filter, setFilter] = useState('')

  useEffect(() => {
    fetchPapers(1, 50)
  }, [fetchPapers])

  const filtered = useMemo<Paper[]>(() => {
    const q = filter.trim().toLowerCase()
    if (!q) return papers
    return papers.filter((p) => {
      return (
        p.title.toLowerCase().includes(q) ||
        p.authors.some((a) => a.toLowerCase().includes(q)) ||
        p.venue.toLowerCase().includes(q) ||
        p.keywords.some((k) => k.toLowerCase().includes(q))
      )
    })
  }, [papers, filter])

  const handleAdd = async () => {
    const title = window.prompt('Paper title?')
    if (!title) return
    const authorsStr = window.prompt('Authors (comma separated)?') || ''
    const authors = authorsStr
      .split(',')
      .map((a) => a.trim())
      .filter(Boolean)
    const venue = window.prompt('Venue (optional)?') || ''
    await createPaper({
      title,
      authors,
      venue,
      abstract: '',
      publicationDate: new Date().toISOString().slice(0, 10),
      url: '',
      keywords: [],
      references: [],
    })
  }

  const handleDelete = async (id: string) => {
    if (window.confirm('Delete this paper?')) {
      await deletePaper(id)
    }
  }

  return (
    <div>
      <h1 style={headingStyle}>Papers</h1>
      <div style={topRowStyle}>
        <input
          type="text"
          placeholder="Search papers..."
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          style={searchInputStyle}
        />
        <button type="button" style={addButtonStyle} onClick={handleAdd}>
          Add paper (manual)
        </button>
      </div>
      {error && (
        <div style={{ ...messageStyle, color: '#e94560', background: '#2a1a2a' }}>{error}</div>
      )}
      {loading && papers.length === 0 ? (
        <div style={emptyStateStyle}>Loading papers…</div>
      ) : filtered.length === 0 ? (
        <div style={emptyStateStyle}>
          {filter ? 'No papers match your search.' : 'No papers yet. Import a paper to get started.'}
        </div>
      ) : (
        <div style={listStyle}>
          {filtered.map((paper) => (
            <div key={paper.id} style={paperCardStyle}>
              <div style={{ flex: 1, minWidth: 0 }}>
                <h3 style={paperTitleStyle}>{paper.title}</h3>
                <p style={paperMetaStyle}>
                  {paper.authors.length > 0 ? paper.authors.join(', ') : 'Unknown author'}
                  {paper.venue ? ` · ${paper.venue}` : ''}
                  {paper.publicationDate ? ` · ${paper.publicationDate}` : ''}
                  {paper.citations ? ` · ${paper.citations} citations` : ''}
                </p>
              </div>
              <button type="button" style={secondaryButtonStyle} onClick={() => handleDelete(paper.id)}>
                Delete
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}

export default Papers
