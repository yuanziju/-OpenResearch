import { create } from 'zustand'
import { searchApi, ApiError } from '@/client/api/client'
import { MAX_SEARCH_RESULTS } from '@/shared/constants'
import type { Paper, SearchFilters, SearchQuery } from '@/shared/types'

interface SearchState {
  query: string
  filters: SearchFilters
  results: Paper[]
  total: number
  sources: string[]
  loading: boolean
  error: string | null
  hasSearched: boolean
  savedQueries: SearchQuery[]
  setQuery: (q: string) => void
  setFilters: (f: Partial<SearchFilters>) => void
  executeSearch: (page?: number, limit?: number) => Promise<void>
  fetchSavedQueries: () => Promise<void>
  saveCurrentQuery: () => Promise<SearchQuery | null>
  deleteSavedQuery: (id: string) => Promise<boolean>
  clearError: () => void
}

function errorMessage(err: unknown, fallback: string): string {
  if (err instanceof ApiError) {
    return err.message || fallback
  }
  return err instanceof Error ? err.message : fallback
}

const emptyFilters: SearchFilters = {}

export const useSearchStore = create<SearchState>((set, get) => ({
  query: '',
  filters: emptyFilters,
  results: [],
  total: 0,
  sources: [],
  loading: false,
  error: null,
  hasSearched: false,
  savedQueries: [],

  setQuery: (q) => set({ query: q }),

  setFilters: (f) =>
    set((state) => ({ filters: { ...state.filters, ...f } })),

  executeSearch: async (page = 1, limit = MAX_SEARCH_RESULTS) => {
    const { query, filters } = get()
    set({ loading: true, error: null })
    try {
      const res = await searchApi.search(query, filters, page, limit)
      set({
        results: res.results,
        total: res.total,
        sources: res.sources,
        loading: false,
        hasSearched: true,
      })
    } catch (err) {
      set({
        loading: false,
        error: errorMessage(err, 'Search failed'),
        hasSearched: true,
      })
    }
  },

  fetchSavedQueries: async () => {
    set({ error: null })
    try {
      const res = await searchApi.listSaved()
      set({ savedQueries: res.queries })
    } catch (err) {
      set({ error: errorMessage(err, 'Failed to load saved queries') })
    }
  },

  saveCurrentQuery: async () => {
    const { query, filters } = get()
    set({ error: null })
    try {
      const res = await searchApi.saveSaved(query, filters)
      set((state) => ({ savedQueries: [res.query, ...state.savedQueries] }))
      return res.query
    } catch (err) {
      set({ error: errorMessage(err, 'Failed to save query') })
      return null
    }
  },

  deleteSavedQuery: async (id) => {
    set({ error: null })
    try {
      await searchApi.removeSaved(id)
      set((state) => ({ savedQueries: state.savedQueries.filter((q) => q.id !== id) }))
      return true
    } catch (err) {
      set({ error: errorMessage(err, 'Failed to delete saved query') })
      return false
    }
  },

  clearError: () => set({ error: null }),
}))
