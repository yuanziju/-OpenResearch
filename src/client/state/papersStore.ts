import { create } from 'zustand'
import { papersApi, ApiError, type ListPapersParams } from '@/client/api/client'
import type { Paper } from '@/shared/types'

interface PapersState {
  papers: Paper[]
  total: number
  page: number
  limit: number
  loading: boolean
  error: string | null
  fetchPapers: (page?: number, limit?: number, params?: ListPapersParams) => Promise<void>
  createPaper: (data: Partial<Paper>) => Promise<Paper | null>
  updatePaper: (id: string, data: Partial<Paper>) => Promise<Paper | null>
  deletePaper: (id: string) => Promise<boolean>
  clearError: () => void
}

function errorMessage(err: unknown, fallback: string): string {
  if (err instanceof ApiError) {
    return err.message || fallback
  }
  return err instanceof Error ? err.message : fallback
}

export const usePapersStore = create<PapersState>((set) => ({
  papers: [],
  total: 0,
  page: 1,
  limit: 20,
  loading: false,
  error: null,

  fetchPapers: async (page = 1, limit = 20, params) => {
    set({ loading: true, error: null })
    try {
      const res = await papersApi.list({ page, limit, ...(params || {}) })
      set({
        papers: res.papers,
        total: res.total,
        page: res.page,
        limit: res.limit,
        loading: false,
      })
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to load papers') })
    }
  },

  createPaper: async (data) => {
    set({ loading: true, error: null })
    try {
      const res = await papersApi.create(data)
      set((state) => ({
        papers: [res.paper, ...state.papers],
        total: state.total + 1,
        loading: false,
      }))
      return res.paper
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to create paper') })
      return null
    }
  },

  updatePaper: async (id, data) => {
    set({ loading: true, error: null })
    try {
      const res = await papersApi.update(id, data)
      set((state) => ({
        papers: state.papers.map((p) => (p.id === id ? res.paper : p)),
        loading: false,
      }))
      return res.paper
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to update paper') })
      return null
    }
  },

  deletePaper: async (id) => {
    set({ loading: true, error: null })
    try {
      await papersApi.remove(id)
      set((state) => ({
        papers: state.papers.filter((p) => p.id !== id),
        total: Math.max(0, state.total - 1),
        loading: false,
      }))
      return true
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to delete paper') })
      return false
    }
  },

  clearError: () => set({ error: null }),
}))
