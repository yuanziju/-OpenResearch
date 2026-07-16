import { useEffect, useRef, useState } from 'react'
import { useNotesStore } from '@/client/state'

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
  display: 'flex',
  flexDirection: 'column',
}

const rightColumnStyle: React.CSSProperties = {
  flex: 1,
  background: '#16213e',
  borderRadius: '8px',
  padding: '16px',
  boxSizing: 'border-box',
  display: 'flex',
  flexDirection: 'column',
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

const noteListStyle: React.CSSProperties = {
  display: 'flex',
  flexDirection: 'column',
  gap: '4px',
  overflowY: 'auto',
  flex: 1,
}

const noteItemStyle: React.CSSProperties = {
  padding: '10px 12px',
  borderRadius: '6px',
  cursor: 'pointer',
  border: '1px solid transparent',
}

const noteItemActiveStyle: React.CSSProperties = {
  background: '#0f3460',
  border: '1px solid #e94560',
}

const noteTitleStyle: React.CSSProperties = {
  fontSize: '14px',
  color: '#e0e0e0',
  margin: '0 0 4px 0',
  whiteSpace: 'nowrap',
  overflow: 'hidden',
  textOverflow: 'ellipsis',
}

const noteDateStyle: React.CSSProperties = {
  fontSize: '11px',
  color: '#a0a0b0',
  margin: 0,
}

const emptyStateStyle: React.CSSProperties = {
  color: '#a0a0b0',
  fontSize: '16px',
}

const editorTitleInputStyle: React.CSSProperties = {
  width: '100%',
  padding: '10px 12px',
  fontSize: '16px',
  fontWeight: 'bold',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: '#1a1a2e',
  color: '#e0e0e0',
  boxSizing: 'border-box',
  marginBottom: '12px',
}

const editorTextareaStyle: React.CSSProperties = {
  flex: 1,
  width: '100%',
  padding: '12px',
  fontSize: '14px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: '#1a1a2e',
  color: '#e0e0e0',
  boxSizing: 'border-box',
  resize: 'none',
  fontFamily: 'inherit',
  minHeight: '300px',
}

const tagsInputStyle: React.CSSProperties = {
  width: '100%',
  padding: '8px 10px',
  fontSize: '13px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: '#1a1a2e',
  color: '#e0e0e0',
  boxSizing: 'border-box',
  marginBottom: '12px',
}

const metaStyle: React.CSSProperties = {
  fontSize: '12px',
  color: '#a0a0b0',
  marginBottom: '12px',
}

const messageStyle: React.CSSProperties = {
  padding: '8px 12px',
  borderRadius: '6px',
  fontSize: '13px',
  marginBottom: '12px',
}

const secondaryButtonStyle: React.CSSProperties = {
  padding: '6px 12px',
  fontSize: '12px',
  borderRadius: '6px',
  border: '1px solid #2a2a4a',
  background: 'transparent',
  color: '#a0a0b0',
  cursor: 'pointer',
  marginTop: '12px',
}

function formatDate(iso: string): string {
  if (!iso) return ''
  try {
    return new Date(iso).toLocaleString()
  } catch {
    return iso
  }
}

function Notes() {
  const notes = useNotesStore((s) => s.notes)
  const currentNote = useNotesStore((s) => s.currentNote)
  const loading = useNotesStore((s) => s.loading)
  const error = useNotesStore((s) => s.error)
  const fetchNotes = useNotesStore((s) => s.fetchNotes)
  const selectNote = useNotesStore((s) => s.selectNote)
  const createNote = useNotesStore((s) => s.createNote)
  const updateNote = useNotesStore((s) => s.updateNote)
  const deleteNote = useNotesStore((s) => s.deleteNote)

  const [title, setTitle] = useState('')
  const [content, setContent] = useState('')
  const [tags, setTags] = useState('')
  const lastLoadedIdRef = useRef<string | null>(null)

  useEffect(() => {
    fetchNotes()
  }, [fetchNotes])

  // When the selected note changes, hydrate the editor fields from it.
  useEffect(() => {
    if (currentNote && currentNote.id !== lastLoadedIdRef.current) {
      setTitle(currentNote.title)
      setContent(currentNote.content)
      setTags(currentNote.tags.join(', '))
      lastLoadedIdRef.current = currentNote.id
    }
    if (!currentNote) {
      lastLoadedIdRef.current = null
    }
  }, [currentNote])

  const handleNewNote = async () => {
    await createNote({ title: 'Untitled note', content: '', tags: [] })
  }

  const handleSelect = (id: string) => {
    selectNote(id)
  }

  const persist = async (field: 'title' | 'content' | 'tags') => {
    if (!currentNote) return
    const patch: Record<string, unknown> = {}
    if (field === 'title') patch.title = title
    if (field === 'content') patch.content = content
    if (field === 'tags') {
      patch.tags = tags
        .split(',')
        .map((t) => t.trim())
        .filter(Boolean)
    }
    await updateNote(currentNote.id, patch)
  }

  const handleDelete = async () => {
    if (!currentNote) return
    if (window.confirm('Delete this note?')) {
      await deleteNote(currentNote.id)
    }
  }

  return (
    <div>
      <h1 style={headingStyle}>Notes</h1>
      <div style={columnsStyle}>
        <div style={leftColumnStyle}>
          <button type="button" style={newNoteButtonStyle} onClick={handleNewNote}>
            New Note
          </button>
          {loading && notes.length === 0 ? (
            <div style={{ ...emptyStateStyle, fontSize: '14px' }}>Loading notes…</div>
          ) : notes.length === 0 ? (
            <div style={{ ...emptyStateStyle, fontSize: '14px' }}>No notes yet.</div>
          ) : (
            <div style={noteListStyle}>
              {notes.map((note) => {
                const active = currentNote?.id === note.id
                return (
                  <div
                    key={note.id}
                    style={active ? { ...noteItemStyle, ...noteItemActiveStyle } : noteItemStyle}
                    onClick={() => handleSelect(note.id)}
                  >
                    <p style={noteTitleStyle}>{note.title || 'Untitled'}</p>
                    <p style={noteDateStyle}>{formatDate(note.updatedAt)}</p>
                  </div>
                )
              })}
            </div>
          )}
        </div>
        <div style={rightColumnStyle}>
          {error && (
            <div style={{ ...messageStyle, color: '#e94560', background: '#2a1a2a' }}>{error}</div>
          )}
          {currentNote ? (
            <>
              <input
                type="text"
                style={editorTitleInputStyle}
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                onBlur={() => persist('title')}
                placeholder="Note title"
              />
              <div style={metaStyle}>
                Updated {formatDate(currentNote.updatedAt)} · Created {formatDate(currentNote.createdAt)}
              </div>
              <input
                type="text"
                style={tagsInputStyle}
                value={tags}
                onChange={(e) => setTags(e.target.value)}
                onBlur={() => persist('tags')}
                placeholder="Tags (comma separated)"
              />
              <textarea
                style={editorTextareaStyle}
                value={content}
                onChange={(e) => setContent(e.target.value)}
                onBlur={() => persist('content')}
                placeholder="Write your note…"
              />
              <button type="button" style={secondaryButtonStyle} onClick={handleDelete}>
                Delete note
              </button>
            </>
          ) : (
            <div
              style={{
                ...emptyStateStyle,
                flex: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              Select or create a note
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default Notes
