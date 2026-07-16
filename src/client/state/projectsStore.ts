import { create } from 'zustand'
import { projectsApi, ApiError } from '@/client/api/client'
import type { Project } from '@/shared/types'

interface ProjectsState {
  projects: Project[]
  currentProject: Project | null
  loading: boolean
  error: string | null
  fetchProjects: () => Promise<void>
  createProject: (name: string, description: string) => Promise<Project | null>
  updateProject: (id: string, data: Partial<Project>) => Promise<Project | null>
  deleteProject: (id: string) => Promise<boolean>
  clearError: () => void
}

function errorMessage(err: unknown, fallback: string): string {
  if (err instanceof ApiError) {
    return err.message || fallback
  }
  return err instanceof Error ? err.message : fallback
}

export const useProjectsStore = create<ProjectsState>((set) => ({
  projects: [],
  currentProject: null,
  loading: false,
  error: null,

  fetchProjects: async () => {
    set({ loading: true, error: null })
    try {
      const res = await projectsApi.list()
      set({ projects: res.projects, loading: false })
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to load projects') })
    }
  },

  createProject: async (name, description) => {
    set({ loading: true, error: null })
    try {
      const res = await projectsApi.create(name, description)
      set((state) => ({
        projects: [res.project, ...state.projects],
        loading: false,
      }))
      return res.project
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to create project') })
      return null
    }
  },

  updateProject: async (id, data) => {
    set({ loading: true, error: null })
    try {
      const res = await projectsApi.update(id, data)
      set((state) => ({
        projects: state.projects.map((p) => (p.id === id ? res.project : p)),
        currentProject: state.currentProject && state.currentProject.id === id ? res.project : state.currentProject,
        loading: false,
      }))
      return res.project
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to update project') })
      return null
    }
  },

  deleteProject: async (id) => {
    set({ loading: true, error: null })
    try {
      await projectsApi.remove(id)
      set((state) => ({
        projects: state.projects.filter((p) => p.id !== id),
        currentProject: state.currentProject && state.currentProject.id === id ? null : state.currentProject,
        loading: false,
      }))
      return true
    } catch (err) {
      set({ loading: false, error: errorMessage(err, 'Failed to delete project') })
      return false
    }
  },

  clearError: () => set({ error: null }),
}))
