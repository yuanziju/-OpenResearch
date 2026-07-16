export interface Paper {
  id: string
  title: string
  authors: string[]
  abstract: string
  publicationDate: string
  venue: string
  citations: number
  url: string
  pdfUrl?: string
  keywords: string[]
  references: string[]
}

export interface Note {
  id: string
  title: string
  content: string
  createdAt: string
  updatedAt: string
  tags: string[]
  paperId?: string
}

export interface Project {
  id: string
  name: string
  description: string
  createdAt: string
  updatedAt: string
  members: User[]
}

export interface User {
  id: string
  name: string
  email: string
  avatar?: string
}

export interface SearchQuery {
  id: string
  query: string
  filters: SearchFilters
  createdAt: string
  saved: boolean
}

export interface SearchFilters {
  dateRange?: { start: string; end: string }
  venues?: string[]
  authors?: string[]
  keywords?: string[]
}

export interface AIResponse {
  summary: string
  keyInsights: string[]
  relatedPapers: string[]
}

export interface Citation {
  id: string
  paperId: string
  format: 'bibtex' | 'endnote' | 'apa' | 'mla' | 'chicago'
  content: string
}

export interface GraphNode {
  id: string
  label: string
  type: 'paper' | 'author' | 'concept' | 'venue'
}

export interface GraphEdge {
  source: string
  target: string
  type: 'cites' | 'authors' | 'about' | 'publishedIn'
}
