import { create } from 'zustand'
import { notesApi, ApiError, type CreateNotePayload } from '@/client/api/client'
import type { Note } from '@/shared/types'

interface NotesState {
  notes: Note[]
  currentNote: Note | null
  loading: boolean
  error: string | null
  fetchNotes: (paperId?: string) => Promise<void>
  selectNote: (id: string | null) => void
  createNote: (payload: CreateNotePayload) => Promise<Note | null>
  updateNote: (id: string, data: Partial<Note>) => Promise<Note | null>
  deleteNote: (id: string) => Promise<boolean>
  clearError: () => void
}

function errorMessage(err: unknown, fallback: string): string {
  if (err instanceof ApiError) {
    return err.message || fallback
  }
  return err instanceof Error ? err.message : fallback
}

export const useNotesStore = create<NotesState>((set, get) => ({
  notes: [],
  currentNote: null,
  loading: false,
  error: null,

  fetchNotes: async (paperId) => {
    set({ loading: true, error: null })
    try {
      const res = await notesApi.list(paperId)
      set({ notes: res.notes, loading: false })
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to load notes') })
    }
  },

  selectNote: (id) => {
    if (id === null) {
      set({ currentNote: null })
      return
    }
    const note = get().notes.find((n) => n.id === id) || null
    set({ currentNote: note })
  },

  createNote: async (payload) => {
    set({ loading: true, error: null })
    try {
      const res = await notesApi.create(payload)
      set((state) => ({
        notes: [res.note, ...state.notes],
        currentNote: res.note,
        loading: false,
      }))
      return res.note
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to create note') })
      return null
    }
  },

  updateNote: async (id, data) => {
    // Optimistically clear error but keep loading lightweight so the editor
    // stays responsive during autosave.
    set({ error: null })
    try {
      const res = await notesApi.update(id, data)
      set((state) => ({
        notes: state.notes.map((n) => (n.id === id ? res.note : n)),
        currentNote: state.currentNote && state.currentNote.id === id ? res.note : state.currentNote,
      }))
      return res.note
    } catch (err) {
      set({ error: errorMessage(err, 'Failed to update note') })
      return null
    }
  },

  deleteNote: async (id) => {
    set({ loading: true, error: null })
    try {
      await notesApi.remove(id)
      set((state) => ({
        notes: state.notes.filter((n) => n.id !== id),
        currentNote: state.currentNote && state.currentNote.id === id ? null : state.currentNote,
        loading: false,
      }))
      return true
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to delete note') })
      return false
    }
  },

  clearError: () => set({ error: null }),
}))
