import { randomUUID } from 'crypto';
import database from './index';
import type {
  Citation,
  Note,
  Paper,
  Project,
  SearchFilters,
  SearchQuery,
  User,
} from '../../shared/types';

const db = database.getDb();

// ---------------------------------------------------------------------------
// Row types (snake_case as stored in SQLite) and conversion helpers
// ---------------------------------------------------------------------------

interface PaperRow {
  id: string;
  title: string;
  authors: string;
  abstract: string | null;
  publication_date: string | null;
  venue: string | null;
  citations: number;
  url: string | null;
  pdf_url: string | null;
  keywords: string | null;
  references: string | null;
}

interface NoteRow {
  id: string;
  title: string;
  content: string | null;
  created_at: string;
  updated_at: string;
  tags: string | null;
  paper_id: string | null;
}

interface ProjectRow {
  id: string;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
}

function parseStringArray(value: string | null): string[] {
  if (!value) return [];
  try {
    const parsed: unknown = JSON.parse(value);
    return Array.isArray(parsed) ? (parsed as string[]) : [];
  } catch {
    return [];
  }
}

function rowToPaper(row: PaperRow): Paper {
  return {
    id: row.id,
    title: row.title,
    authors: parseStringArray(row.authors),
    abstract: row.abstract ?? '',
    publicationDate: row.publication_date ?? '',
    venue: row.venue ?? '',
    citations: row.citations,
    url: row.url ?? '',
    pdfUrl: row.pdf_url ?? undefined,
    keywords: parseStringArray(row.keywords),
    references: parseStringArray(row.references),
  };
}

function rowToNote(row: NoteRow): Note {
  return {
    id: row.id,
    title: row.title,
    content: row.content ?? '',
    createdAt: row.created_at,
    updatedAt: row.updated_at,
    tags: parseStringArray(row.tags),
    paperId: row.paper_id ?? undefined,
  };
}

function rowToProject(row: ProjectRow): Project {
  return {
    id: row.id,
    name: row.name,
    description: row.description ?? '',
    createdAt: row.created_at,
    updatedAt: row.updated_at,
    // Members live in the project_members table and are populated by route handlers.
    members: [],
  };
}

// ---------------------------------------------------------------------------
// Papers
// ---------------------------------------------------------------------------

export function getPapers(
  page: number = 1,
  limit: number = 20,
): { papers: Paper[]; total: number } {
  const offset = (page - 1) * limit;
  const rows = db
    .prepare<PaperRow>(
      'SELECT * FROM papers ORDER BY publication_date DESC LIMIT ? OFFSET ?',
    )
    .all(limit, offset);
  const totalRow = db
    .prepare<{ total: number }>('SELECT COUNT(*) AS total FROM papers')
    .get();
  return { papers: rows.map(rowToPaper), total: totalRow?.total ?? 0 };
}

/**
 * Whitelisted sort columns for the papers listing endpoint. Keys are the
 * public sort names accepted by the API; values are the corresponding SQLite
 * column names. Using a whitelist prevents SQL injection in the ORDER BY
 * clause (column identifiers cannot be parameterized).
 */
const PAPER_SORT_COLUMNS: Record<string, string> = {
  title: 'title',
  publicationDate: 'publication_date',
  date: 'publication_date',
  citations: 'citations',
  venue: 'venue',
  url: 'url',
};

/**
 * Paginated paper listing with configurable sorting. `sort` must be one of
 * the keys in `PAPER_SORT_COLUMNS` (defaults to publication_date); `order`
 * is 'asc' or 'desc' (defaults to 'desc').
 */
export function getPapersPaged(
  page: number = 1,
  limit: number = 20,
  sort: string = 'publicationDate',
  order: 'asc' | 'desc' = 'desc',
): { papers: Paper[]; total: number } {
  const column = PAPER_SORT_COLUMNS[sort] ?? 'publication_date';
  const direction = order === 'asc' ? 'ASC' : 'DESC';
  const offset = (page - 1) * limit;
  const rows = db
    .prepare<PaperRow>(
      `SELECT * FROM papers ORDER BY ${column} ${direction} LIMIT ? OFFSET ?`,
    )
    .all(limit, offset);
  const totalRow = db
    .prepare<{ total: number }>('SELECT COUNT(*) AS total FROM papers')
    .get();
  return { papers: rows.map(rowToPaper), total: totalRow?.total ?? 0 };
}

export function getPaperById(id: string): Paper | null {
  const row = db.prepare<PaperRow>('SELECT * FROM papers WHERE id = ?').get(id);
  return row ? rowToPaper(row) : null;
}

export function createPaper(paper: Omit<Paper, 'id'>): Paper {
  const id = randomUUID();
  db.prepare(
    `INSERT INTO papers
      (id, title, authors, abstract, publication_date, venue, citations, url, pdf_url, keywords, "references")
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
  ).run(
    id,
    paper.title,
    JSON.stringify(paper.authors),
    paper.abstract ?? null,
    paper.publicationDate ?? null,
    paper.venue ?? null,
    paper.citations ?? 0,
    paper.url ?? null,
    paper.pdfUrl ?? null,
    paper.keywords ? JSON.stringify(paper.keywords) : null,
    paper.references ? JSON.stringify(paper.references) : null,
  );
  return getPaperById(id)!;
}

export function updatePaper(id: string, data: Partial<Paper>): Paper | null {
  const existing = getPaperById(id);
  if (!existing) return null;

  const fields: string[] = [];
  const values: unknown[] = [];

  if (data.title !== undefined) {
    fields.push('title = ?');
    values.push(data.title);
  }
  if (data.authors !== undefined) {
    fields.push('authors = ?');
    values.push(JSON.stringify(data.authors));
  }
  if (data.abstract !== undefined) {
    fields.push('abstract = ?');
    values.push(data.abstract);
  }
  if (data.publicationDate !== undefined) {
    fields.push('publication_date = ?');
    values.push(data.publicationDate);
  }
  if (data.venue !== undefined) {
    fields.push('venue = ?');
    values.push(data.venue);
  }
  if (data.citations !== undefined) {
    fields.push('citations = ?');
    values.push(data.citations);
  }
  if (data.url !== undefined) {
    fields.push('url = ?');
    values.push(data.url);
  }
  if (data.pdfUrl !== undefined) {
    fields.push('pdf_url = ?');
    values.push(data.pdfUrl);
  }
  if (data.keywords !== undefined) {
    fields.push('keywords = ?');
    values.push(JSON.stringify(data.keywords));
  }
  if (data.references !== undefined) {
    fields.push('"references" = ?');
    values.push(JSON.stringify(data.references));
  }

  if (fields.length === 0) {
    return existing;
  }

  values.push(id);
  db.prepare(`UPDATE papers SET ${fields.join(', ')} WHERE id = ?`).run(
    ...values,
  );

  return getPaperById(id);
}

export function deletePaper(id: string): boolean {
  const result = db.prepare('DELETE FROM papers WHERE id = ?').run(id);
  return result.changes > 0;
}

// ---------------------------------------------------------------------------
// Notes
// ---------------------------------------------------------------------------

export function getNotes(paperId?: string): Note[] {
  const rows = paperId
    ? db
        .prepare<NoteRow>(
          'SELECT * FROM notes WHERE paper_id = ? ORDER BY created_at DESC',
        )
        .all(paperId)
    : db
        .prepare<NoteRow>('SELECT * FROM notes ORDER BY created_at DESC')
        .all();
  return rows.map(rowToNote);
}

export function getNoteById(id: string): Note | null {
  const row = db.prepare<NoteRow>('SELECT * FROM notes WHERE id = ?').get(id);
  return row ? rowToNote(row) : null;
}

export function createNote(
  note: Omit<Note, 'id' | 'createdAt' | 'updatedAt'>,
): Note {
  const id = randomUUID();
  const now = new Date().toISOString();
  db.prepare(
    `INSERT INTO notes
      (id, title, content, created_at, updated_at, tags, paper_id)
    VALUES (?, ?, ?, ?, ?, ?, ?)`,
  ).run(
    id,
    note.title,
    note.content ?? null,
    now,
    now,
    note.tags ? JSON.stringify(note.tags) : null,
    note.paperId ?? null,
  );
  return getNoteById(id)!;
}

export function updateNote(id: string, data: Partial<Note>): Note | null {
  const existing = getNoteById(id);
  if (!existing) return null;

  const fields: string[] = [];
  const values: unknown[] = [];

  if (data.title !== undefined) {
    fields.push('title = ?');
    values.push(data.title);
  }
  if (data.content !== undefined) {
    fields.push('content = ?');
    values.push(data.content);
  }
  if (data.tags !== undefined) {
    fields.push('tags = ?');
    values.push(JSON.stringify(data.tags));
  }
  if (data.paperId !== undefined) {
    fields.push('paper_id = ?');
    values.push(data.paperId);
  }

  // updatedAt is always refreshed on a note edit.
  fields.push('updated_at = ?');
  values.push(new Date().toISOString());

  values.push(id);
  db.prepare(`UPDATE notes SET ${fields.join(', ')} WHERE id = ?`).run(
    ...values,
  );

  return getNoteById(id);
}

export function deleteNote(id: string): boolean {
  const result = db.prepare('DELETE FROM notes WHERE id = ?').run(id);
  return result.changes > 0;
}

// ---------------------------------------------------------------------------
// Projects
// ---------------------------------------------------------------------------

export function getProjects(): Project[] {
  const rows = db
    .prepare<ProjectRow>('SELECT * FROM projects ORDER BY created_at DESC')
    .all();
  return rows.map(rowToProject);
}

export function getProjectById(id: string): Project | null {
  const row = db
    .prepare<ProjectRow>('SELECT * FROM projects WHERE id = ?')
    .get(id);
  return row ? rowToProject(row) : null;
}

export function createProject(
  project: Omit<Project, 'id' | 'createdAt' | 'updatedAt'>,
): Project {
  const id = randomUUID();
  const now = new Date().toISOString();
  db.prepare(
    `INSERT INTO projects
      (id, name, description, created_at, updated_at)
    VALUES (?, ?, ?, ?, ?)`,
  ).run(id, project.name, project.description ?? null, now, now);
  return getProjectById(id)!;
}

export function updateProject(
  id: string,
  data: Partial<Project>,
): Project | null {
  const existing = getProjectById(id);
  if (!existing) return null;

  const fields: string[] = [];
  const values: unknown[] = [];

  if (data.name !== undefined) {
    fields.push('name = ?');
    values.push(data.name);
  }
  if (data.description !== undefined) {
    fields.push('description = ?');
    values.push(data.description);
  }

  // updatedAt is always refreshed on a project edit.
  fields.push('updated_at = ?');
  values.push(new Date().toISOString());

  values.push(id);
  db.prepare(`UPDATE projects SET ${fields.join(', ')} WHERE id = ?`).run(
    ...values,
  );

  return getProjectById(id);
}

export function deleteProject(id: string): boolean {
  const result = db.prepare('DELETE FROM projects WHERE id = ?').run(id);
  return result.changes > 0;
}

// ---------------------------------------------------------------------------
// Project members (project_members table)
// ---------------------------------------------------------------------------

// The local schema stores only user_id in project_members; a full users/auth
// module does not exist yet, so members are surfaced with the id and empty
// profile fields. This keeps the response shape consistent with the `User`
// type used throughout the API.
interface ProjectMemberRow {
  project_id: string;
  user_id: string;
}

function rowToUser(row: ProjectMemberRow): User {
  return { id: row.user_id, name: '', email: '' };
}

export function getProjectMembers(projectId: string): User[] {
  const rows = db
    .prepare<ProjectMemberRow>(
      'SELECT project_id, user_id FROM project_members WHERE project_id = ?',
    )
    .all(projectId);
  return rows.map(rowToUser);
}

export function addProjectMember(projectId: string, userId: string): boolean {
  // INSERT OR IGNORE keeps adding an existing member idempotent.
  const result = db
    .prepare(
      'INSERT OR IGNORE INTO project_members (project_id, user_id) VALUES (?, ?)',
    )
    .run(projectId, userId);
  return result.changes > 0;
}

export function removeProjectMember(projectId: string, userId: string): boolean {
  const result = db
    .prepare(
      'DELETE FROM project_members WHERE project_id = ? AND user_id = ?',
    )
    .run(projectId, userId);
  return result.changes > 0;
}

export function deleteProjectMembers(projectId: string): void {
  db.prepare('DELETE FROM project_members WHERE project_id = ?').run(projectId);
}

// ---------------------------------------------------------------------------
// Saved searches (search_queries table)
// ---------------------------------------------------------------------------

interface SearchQueryRow {
  id: string;
  query: string;
  filters: string | null;
  created_at: string;
  saved: number;
}

function parseFilters(value: string | null): SearchFilters {
  if (!value) return {};
  try {
    const parsed: unknown = JSON.parse(value);
    if (parsed && typeof parsed === 'object') {
      return parsed as SearchFilters;
    }
  } catch {
    /* fall through to empty filters */
  }
  return {};
}

function rowToSearchQuery(row: SearchQueryRow): SearchQuery {
  return {
    id: row.id,
    query: row.query,
    filters: parseFilters(row.filters),
    createdAt: row.created_at,
    saved: row.saved === 1,
  };
}

export function getSavedSearches(): SearchQuery[] {
  const rows = db
    .prepare<SearchQueryRow>(
      'SELECT * FROM search_queries WHERE saved = 1 ORDER BY created_at DESC',
    )
    .all();
  return rows.map(rowToSearchQuery);
}

export function getSavedSearchById(id: string): SearchQuery | null {
  const row = db
    .prepare<SearchQueryRow>('SELECT * FROM search_queries WHERE id = ?')
    .get(id);
  return row ? rowToSearchQuery(row) : null;
}

export function createSavedSearch(
  query: string,
  filters: SearchFilters,
): SearchQuery {
  const id = randomUUID();
  const now = new Date().toISOString();
  db.prepare(
    'INSERT INTO search_queries (id, query, filters, created_at, saved) VALUES (?, ?, ?, ?, 1)',
  ).run(id, query, JSON.stringify(filters ?? {}), now);
  return getSavedSearchById(id)!;
}

export function deleteSavedSearch(id: string): boolean {
  const result = db
    .prepare('DELETE FROM search_queries WHERE id = ?')
    .run(id);
  return result.changes > 0;
}

// ---------------------------------------------------------------------------
// Citations (citations table)
// ---------------------------------------------------------------------------

interface CitationRow {
  id: string;
  paper_id: string;
  format: string;
  content: string;
}

function rowToCitation(row: CitationRow): Citation {
  return {
    id: row.id,
    paperId: row.paper_id,
    format: row.format as Citation['format'],
    content: row.content,
  };
}

export function getCitationsByPaper(paperId: string): Citation[] {
  const rows = db
    .prepare<CitationRow>(
      'SELECT * FROM citations WHERE paper_id = ? ORDER BY format',
    )
    .all(paperId);
  return rows.map(rowToCitation);
}

export function getCitationById(id: string): Citation | null {
  const row = db
    .prepare<CitationRow>('SELECT * FROM citations WHERE id = ?')
    .get(id);
  return row ? rowToCitation(row) : null;
}

export function createCitation(
  paperId: string,
  format: Citation['format'],
  content: string,
): Citation {
  const id = randomUUID();
  db.prepare(
    'INSERT INTO citations (id, paper_id, format, content) VALUES (?, ?, ?, ?)',
  ).run(id, paperId, format, content);
  return getCitationById(id)!;
}

export function getAllCitationsForExport(
  paperIds: string[],
  format?: Citation['format'],
): Citation[] {
  if (paperIds.length === 0) return [];
  const placeholders = paperIds.map(() => '?').join(', ');
  const sql = format
    ? `SELECT * FROM citations WHERE paper_id IN (${placeholders}) AND format = ? ORDER BY paper_id, format`
    : `SELECT * FROM citations WHERE paper_id IN (${placeholders}) ORDER BY paper_id, format`;
  const params = format ? [...paperIds, format] : paperIds;
  const rows = db.prepare<CitationRow>(sql).all(...params);
  return rows.map(rowToCitation);
}

// ---------------------------------------------------------------------------
// Paper search (local DB filtering)
// ---------------------------------------------------------------------------

/**
 * Searches the local papers table. The query string is matched
 * case-insensitively against title, abstract, and the serialized keywords
 * array. Any provided SearchFilters (venues, dateRange, authors, keywords)
 * are applied as additional constraints. Results are paginated and ordered
 * by publication_date descending.
 */
export function searchPapers(
  query: string,
  filters: SearchFilters,
  page: number = 1,
  limit: number = 20,
): { papers: Paper[]; total: number } {
  const conditions: string[] = [];
  const values: unknown[] = [];

  const trimmed = (query ?? '').trim();
  if (trimmed) {
    const like = `%${trimmed}%`;
    conditions.push(
      '(LOWER(title) LIKE LOWER(?) OR LOWER(COALESCE(abstract, "")) LIKE LOWER(?) OR LOWER(COALESCE(keywords, "")) LIKE LOWER(?))',
    );
    values.push(like, like, like);
  }

  if (filters.venues && filters.venues.length > 0) {
    const placeholders = filters.venues.map(() => '?').join(', ');
    conditions.push(`(venue IS NOT NULL AND venue IN (${placeholders}))`);
    values.push(...filters.venues);
  }

  if (filters.dateRange) {
    if (filters.dateRange.start) {
      conditions.push('publication_date >= ?');
      values.push(filters.dateRange.start);
    }
    if (filters.dateRange.end) {
      conditions.push('publication_date <= ?');
      values.push(filters.dateRange.end);
    }
  }

  if (filters.authors && filters.authors.length > 0) {
    const authorClauses = filters.authors.map(
      () => 'LOWER(authors) LIKE LOWER(?)',
    );
    conditions.push(`(${authorClauses.join(' OR ')})`);
    values.push(...filters.authors.map((a) => `%${a}%`));
  }

  if (filters.keywords && filters.keywords.length > 0) {
    const keywordClauses = filters.keywords.map(
      () => 'LOWER(COALESCE(keywords, "")) LIKE LOWER(?)',
    );
    conditions.push(`(${keywordClauses.join(' OR ')})`);
    values.push(...filters.keywords.map((k) => `%${k}%`));
  }

  const where =
    conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';
  const offset = (page - 1) * limit;

  const rows = db
    .prepare<PaperRow>(
      `SELECT * FROM papers ${where} ORDER BY publication_date DESC LIMIT ? OFFSET ?`,
    )
    .all(...values, limit, offset);
  const totalRow = db
    .prepare<{ total: number }>(
      `SELECT COUNT(*) AS total FROM papers ${where}`,
    )
    .get(...values);
  return { papers: rows.map(rowToPaper), total: totalRow?.total ?? 0 };
}
