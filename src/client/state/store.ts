import { create } from 'zustimport { create } from 'zustandimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResultsimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper:import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, contentimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searchingimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export constimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearchingimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching:import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote:import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote: (note) => set((state) => ({ notes: [...state.notes, note] })),
  updateNote: (id, content) =>
    set((state) => ({import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote: (note) => set((state) => ({ notes: [...state.notes, note] })),
  updateNote: (id, content) =>
    set((state) => ({
      notes: state.notes.map((noteimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote: (note) => set((state) => ({ notes: [...state.notes, note] })),
  updateNote: (id, content) =>
    set((state) => ({
      notes: state.notes.map((note) => (note.id === id ? { ...note, ...content } : note)),
    })),
  deleteNote: (id) => set((state) => ({ notes: stateimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote: (note) => set((state) => ({ notes: [...state.notes, note] })),
  updateNote: (id, content) =>
    set((state) => ({
      notes: state.notes.map((note) => (note.id === id ? { ...note, ...content } : note)),
    })),
  deleteNote: (id) => set((state) => ({ notes: state.notes.filter((note) => note.id !==import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote: (note) => set((state) => ({ notes: [...state.notes, note] })),
  updateNote: (id, content) =>
    set((state) => ({
      notes: state.notes.map((note) => (note.id === id ? { ...note, ...content } : note)),
    })),
  deleteNote: (id) => set((state) => ({ notes: state.notes.filter((note) => note.id !== id) })),
  addProject: (project) => set((state) => ({ projects: [...state.projects, project] })),import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote: (note) => set((state) => ({ notes: [...state.notes, note] })),
  updateNote: (id, content) =>
    set((state) => ({
      notes: state.notes.map((note) => (note.id === id ? { ...note, ...content } : note)),
    })),
  deleteNote: (id) => set((state) => ({ notes: state.notes.filter((note) => note.id !== id) })),
  addProject: (project) => set((state) => ({ projects: [...state.projects, project] })),
  selectProject: (project) => set({ currentProject: project }),
  addimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote: (note) => set((state) => ({ notes: [...state.notes, note] })),
  updateNote: (id, content) =>
    set((state) => ({
      notes: state.notes.map((note) => (note.id === id ? { ...note, ...content } : note)),
    })),
  deleteNote: (id) => set((state) => ({ notes: state.notes.filter((note) => note.id !== id) })),
  addProject: (project) => set((state) => ({ projects: [...state.projects, project] })),
  selectProject: (project) => set({ currentProject: project }),
  addSearchQuery: (query) => set((state) => ({ searchQueries: [...state.searchQueries, query] })),
  setSearchResultsimport { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote: (note) => set((state) => ({ notes: [...state.notes, note] })),
  updateNote: (id, content) =>
    set((state) => ({
      notes: state.notes.map((note) => (note.id === id ? { ...note, ...content } : note)),
    })),
  deleteNote: (id) => set((state) => ({ notes: state.notes.filter((note) => note.id !== id) })),
  addProject: (project) => set((state) => ({ projects: [...state.projects, project] })),
  selectProject: (project) => set({ currentProject: project }),
  addSearchQuery: (query) => set((state) => ({ searchQueries: [...state.searchQueries, query] })),
  setSearchResults: (results) => set({ searchResults:import { create } from 'zustand'
import { Paper, Note, Project, SearchQuery } from '@/shared/types'

interface AppState {
  papers: Paper[]
  selectedPaper: Paper | null
  notes: Note[]
  projects: Project[]
  searchQueries: SearchQuery[]
  currentProject: Project | null
  searchResults: Paper[]
  isSearching: boolean

  addPaper: (paper: Paper) => void
  selectPaper: (paper: Paper | null) => void
  addNote: (note: Note) => void
  updateNote: (id: string, content: Partial<Note>) => void
  deleteNote: (id: string) => void
  addProject: (project: Project) => void
  selectProject: (project: Project | null) => void
  addSearchQuery: (query: SearchQuery) => void
  setSearchResults: (results: Paper[]) => void
  setIsSearching: (searching: boolean) => void
}

export const useAppStore = create<AppState>((set) => ({
  papers: [],
  selectedPaper: null,
  notes: [],
  projects: [],
  searchQueries: [],
  currentProject: null,
  searchResults: [],
  isSearching: false,

  addPaper: (paper) => set((state) => ({ papers: [...state.papers, paper] })),
  selectPaper: (paper) => set({ selectedPaper: paper }),
  addNote: (note) => set((state) => ({ notes: [...state.notes, note] })),
  updateNote: (id, content) =>
    set((state) => ({
      notes: state.notes.map((note) => (note.id === id ? { ...note, ...content } : note)),
    })),
  deleteNote: (id) => set((state) => ({ notes: state.notes.filter((note) => note.id !== id) })),
  addProject: (project) => set((state) => ({ projects: [...state.projects, project] })),
  selectProject: (project) => set({ currentProject: project }),
  addSearchQuery: (query) => set((state) => ({ searchQueries: [...state.searchQueries, query] })),
  setSearchResults: (results) => set({ searchResults: results }),
  setIsSearching: