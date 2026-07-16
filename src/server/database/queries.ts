import { randomUUID } from 'crypto';
import database from './index';
import type { Note, Paper, Project } from '../../shared/types';

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
