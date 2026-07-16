import { API_BASE_URL } from '@/shared/constants'
import type {
  Paper,
  Note,
  Project,
  SearchQuery,
  SearchFilters,
  Citation,
  GraphNode,
  GraphEdge,
} from '@/shared/types'

/**
 * Error thrown for any non-2xx API response. The original HTTP status, the
 * server-reported error code (when present), and any extra details are
 * preserved so callers can surface them to the UI.
 */
export class ApiError extends Error {
  status: number
  code: string
  details?: unknown

  constructor(status: number, code: string, message: string, details?: unknown) {
    super(message)
    this.name = 'ApiError'
    this.status = status
    this.code = code
    this.details = details
  }
}

interface ApiErrorBody {
  error?: { code?: string; message?: string; details?: unknown }
}

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
  init?: RequestInit,
): Promise<T> {
  const headers: Record<string, string> = {
    Accept: 'application/json',
  }
  let payload: BodyInit | undefined
  if (body !== undefined && !(body instanceof FormData)) {
    headers['Content-Type'] = 'application/json'
    payload = JSON.stringify(body)
  } else if (body instanceof FormData) {
    payload = body
  }

  let res: Response
  try {
    res = await fetch(`${API_BASE_URL}${path}`, {
      method,
      headers,
      body: payload,
      ...init,
    })
  } catch (networkErr) {
    throw new ApiError(
      0,
      'NETWORK_ERROR',
      networkErr instanceof Error ? networkErr.message : 'Network request failed',
    )
  }

  const contentType = res.headers.get('Content-Type') || ''
  const isJson = contentType.includes('application/json')

  if (!res.ok) {
    let code = 'HTTP_ERROR'
    let message = `Request failed with status ${res.status}`
    let details: unknown
    if (isJson) {
      try {
        const errBody = (await res.json()) as ApiErrorBody
        if (errBody?.error) {
          if (errBody.error.code) code = errBody.error.code
          if (errBody.error.message) message = errBody.error.message
          details = errBody.error.details
        }
      } catch {
        // fall through with defaults
      }
    }
    throw new ApiError(res.status, code, message, details)
  }

  if (isJson) {
    return (await res.json()) as T
  }
  // Non-JSON success response (e.g. binary download) — return the Response so
  // the caller can decide what to do with it.
  return res as unknown as T
}

type QueryValue = string | number | undefined | null
type QueryParams = { [key: string]: QueryValue }

function buildQuery(params: QueryParams): string {
  const sp = new URLSearchParams()
  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined && value !== null && value !== '') {
      sp.append(key, String(value))
    }
  })
  const qs = sp.toString()
  return qs ? `?${qs}` : ''
}

// ---------------------------------------------------------------------------
// Papers
// ---------------------------------------------------------------------------

export interface ListPapersParams {
  page?: number
  limit?: number
  sort?: string
  order?: 'asc' | 'desc'
  [key: string]: string | number | undefined
}

interface ListPapersResponse {
  papers: Paper[]
  total: number
  page: number
  limit: number
}

interface SinglePaperResponse {
  paper: Paper
}

interface SuccessResponse {
  success: boolean
}

export const papersApi = {
  list(params: ListPapersParams = {}): Promise<ListPapersResponse> {
    return request<ListPapersResponse>('GET', `/papers${buildQuery(params)}`)
  },
  get(id: string): Promise<SinglePaperResponse> {
    return request<SinglePaperResponse>('GET', `/papers/${encodeURIComponent(id)}`)
  },
  create(data: Partial<Paper>): Promise<SinglePaperResponse> {
    return request<SinglePaperResponse>('POST', '/papers', {
      source: 'manual',
      data,
    })
  },
  update(id: string, data: Partial<Paper>): Promise<SinglePaperResponse> {
    return request<SinglePaperResponse>('PUT', `/papers/${encodeURIComponent(id)}`, data)
  },
  remove(id: string): Promise<SuccessResponse> {
    return request<SuccessResponse>('DELETE', `/papers/${encodeURIComponent(id)}`)
  },
}

// ---------------------------------------------------------------------------
// Search
// ---------------------------------------------------------------------------

interface SearchResponse {
  results: Paper[]
  total: number
  sources: string[]
}

interface SavedQueriesResponse {
  queries: SearchQuery[]
}

interface SavedQueryResponse {
  query: SearchQuery
}

export const searchApi = {
  search(
    query: string,
    filters: SearchFilters,
    page?: number,
    limit?: number,
  ): Promise<SearchResponse> {
    return request<SearchResponse>('POST', '/search', { query, filters, page, limit })
  },
  listSaved(): Promise<SavedQueriesResponse> {
    return request<SavedQueriesResponse>('GET', '/search/saved')
  },
  saveSaved(query: string, filters: SearchFilters): Promise<SavedQueryResponse> {
    return request<SavedQueryResponse>('POST', '/search/saved', { query, filters })
  },
  removeSaved(id: string): Promise<SuccessResponse> {
    return request<SuccessResponse>('DELETE', `/search/saved/${encodeURIComponent(id)}`)
  },
}

// ---------------------------------------------------------------------------
// Notes
// ---------------------------------------------------------------------------

interface ListNotesResponse {
  notes: Note[]
}

interface SingleNoteResponse {
  note: Note
}

export interface CreateNotePayload {
  title: string
  content: string
  tags?: string[]
  paperId?: string
}

export const notesApi = {
  list(paperId?: string): Promise<ListNotesResponse> {
    return request<ListNotesResponse>('GET', `/notes${buildQuery({ paperId })}`)
  },
  get(id: string): Promise<SingleNoteResponse> {
    return request<SingleNoteResponse>('GET', `/notes/${encodeURIComponent(id)}`)
  },
  create(payload: CreateNotePayload): Promise<SingleNoteResponse> {
    return request<SingleNoteResponse>('POST', '/notes', payload)
  },
  update(id: string, data: Partial<Note>): Promise<SingleNoteResponse> {
    return request<SingleNoteResponse>('PUT', `/notes/${encodeURIComponent(id)}`, data)
  },
  remove(id: string): Promise<SuccessResponse> {
    return request<SuccessResponse>('DELETE', `/notes/${encodeURIComponent(id)}`)
  },
}

// ---------------------------------------------------------------------------
// Projects
// ---------------------------------------------------------------------------

interface ListProjectsResponse {
  projects: Project[]
}

interface SingleProjectResponse {
  project: Project
  members: import('@/shared/types').User[]
  papers: Paper[]
  notes: Note[]
}

export const projectsApi = {
  list(): Promise<ListProjectsResponse> {
    return request<ListProjectsResponse>('GET', '/projects')
  },
  get(id: string): Promise<SingleProjectResponse> {
    return request<SingleProjectResponse>('GET', `/projects/${encodeURIComponent(id)}`)
  },
  create(name: string, description: string): Promise<{ project: Project }> {
    return request<{ project: Project }>('POST', '/projects', { name, description })
  },
  update(id: string, data: Partial<Project>): Promise<{ project: Project }> {
    return request<{ project: Project }>(
      'PUT',
      `/projects/${encodeURIComponent(id)}`,
      data,
    )
  },
  remove(id: string): Promise<SuccessResponse> {
    return request<SuccessResponse>('DELETE', `/projects/${encodeURIComponent(id)}`)
  },
}

// ---------------------------------------------------------------------------
// AI Analysis (server returns 501 Not Implemented — surface as ApiError)
// ---------------------------------------------------------------------------

interface SummarizeResponse {
  summary: string
}

interface InsightsResponse {
  insights: string[]
}

interface RelatedResponse {
  papers: Paper[]
}

interface ReviewResponse {
  review: string
  citations: string[]
}

interface TrendsResponse {
  trends: { topic: string; direction: string; papers: string[] }[]
}

export const aiApi = {
  summarize(paperId: string): Promise<SummarizeResponse> {
    return request<SummarizeResponse>('POST', '/ai/summarize', { paperId })
  },
  insights(paperId: string): Promise<InsightsResponse> {
    return request<InsightsResponse>('POST', '/ai/insights', { paperId })
  },
  related(paperId: string, limit?: number): Promise<RelatedResponse> {
    return request<RelatedResponse>('POST', '/ai/related', { paperId, limit })
  },
  review(paperIds: string[], topic?: string): Promise<ReviewResponse> {
    return request<ReviewResponse>('POST', '/ai/review', { paperIds, topic })
  },
  trends(topic: string, timeframe?: string): Promise<TrendsResponse> {
    return request<TrendsResponse>('POST', '/ai/trends', { topic, timeframe })
  },
}

// ---------------------------------------------------------------------------
// Citations
// ---------------------------------------------------------------------------

interface CitationsListResponse {
  citations: Citation[]
}

interface CitationCreateResponse {
  citation: Citation
}

export type CitationFormat = Citation['format']

export const citationsApi = {
  listForPaper(paperId: string): Promise<CitationsListResponse> {
    return request<CitationsListResponse>(
      'GET',
      `/citations/${encodeURIComponent(paperId)}`,
    )
  },
  create(paperId: string, format: CitationFormat): Promise<CitationCreateResponse> {
    return request<CitationCreateResponse>('POST', '/citations', { paperId, format })
  },
  /**
   * Triggers a browser download for the requested citation export. The server
   * streams back a binary blob; we turn it into an object URL and click a
   * temporary anchor so the user gets a file download.
   */
  async exportCitations(
    format: CitationFormat,
    paperIds: string[],
    filename = 'citations',
  ): Promise<void> {
    const res = await request<Response>(
      'GET',
      `/citations/export${buildQuery({ format, paperIds: paperIds.join(',') })}`,
    )
    const blob = await res.blob()
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `${filename}.${format === 'bibtex' ? 'bib' : format === 'endnote' ? 'enw' : 'txt'}`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  },
}

// ---------------------------------------------------------------------------
// Knowledge Graph
// ---------------------------------------------------------------------------

interface GraphResponse {
  nodes: GraphNode[]
  edges: GraphEdge[]
}

export const graphApi = {
  get(projectId: string): Promise<GraphResponse> {
    return request<GraphResponse>('GET', `/graph/${encodeURIComponent(projectId)}`)
  },
}
