import express from 'express';
import type { Citation, Paper } from '../../shared/types';
import {
  createCitation,
  getCitationsByPaper,
  getPaperById,
} from '../database/queries';
import { asyncHandler, badRequest, notFound } from '../utils/httpError';

export const citationsRouter = express.Router();

const VALID_FORMATS: ReadonlyArray<Citation['format']> = [
  'bibtex',
  'endnote',
  'apa',
  'mla',
  'chicago',
];

function isValidFormat(value: unknown): value is Citation['format'] {
  return (
    typeof value === 'string' &&
    (VALID_FORMATS as readonly string[]).includes(value)
  );
}

// ---------------------------------------------------------------------------
// Citation formatters
// Each formatter produces a real, correctly-structured citation string for its
// style from a Paper's data. They degrade gracefully when fields are missing.
// ---------------------------------------------------------------------------

/** Extracts a 4-digit year from the start of a date string (e.g. "2023-04-01"). */
function extractYear(date: string): string {
  const match = /^(\d{4})/.exec((date ?? '').trim());
  return match ? match[1] : '';
}

/** Splits a full name into a last name and an array of given-name parts. */
function splitName(name: string): { last: string; given: string[] } {
  const parts = name.trim().split(/\s+/).filter(Boolean);
  if (parts.length === 0) return { last: '', given: [] };
  if (parts.length === 1) return { last: parts[0], given: [] };
  return { last: parts[parts.length - 1], given: parts.slice(0, -1) };
}

/** Builds "F. M." style initials from given-name parts. */
function initials(given: string[]): string {
  return given.map((p) => `${p.charAt(0).toUpperCase()}.`).join(' ');
}

/** Ensures a string ends with exactly one period. */
function ensurePeriod(s: string): string {
  const trimmed = s.trim();
  return trimmed.endsWith('.') ? trimmed : `${trimmed}.`;
}

// --- APA (7th ed.) ---

function formatApaAuthor(name: string): string {
  const { last, given } = splitName(name);
  if (!last) return '';
  return given.length === 0 ? last : `${last}, ${initials(given)}`;
}

function formatApaAuthors(authors: string[]): string {
  const list = authors.map(formatApaAuthor).filter(Boolean);
  if (list.length === 0) return '';
  if (list.length === 1) return list[0];
  if (list.length === 2) return `${list[0]}, & ${list[1]}`;
  return `${list.slice(0, -1).join(', ')}, & ${list[list.length - 1]}`;
}

function toApa(paper: Paper): string {
  const authors = formatApaAuthors(paper.authors);
  const year = extractYear(paper.publicationDate) || 'n.d.';
  const parts: string[] = [];
  if (authors) parts.push(authors);
  parts.push(`(${year}).`);
  parts.push(ensurePeriod(paper.title));
  if (paper.venue) parts.push(ensurePeriod(paper.venue));
  if (paper.url) parts.push(`Retrieved from ${paper.url}`);
  return parts.join(' ');
}

// --- BibTeX ---

function bibtexKey(paper: Paper): string {
  const year = extractYear(paper.publicationDate);
  const first = paper.authors[0]?.trim();
  if (first) {
    const last = splitName(first).last;
    const key = `${last}${year}`.toLowerCase().replace(/[^a-z0-9]/g, '');
    if (key) return key;
  }
  return paper.id.replace(/[^a-zA-Z0-9]/g, '');
}

function formatBibtexAuthor(name: string): string {
  const { last, given } = splitName(name);
  if (!last) return '';
  return given.length === 0 ? last : `${last}, ${given.join(' ')}`;
}

function toBibtex(paper: Paper): string {
  const key = bibtexKey(paper);
  const author = paper.authors
    .map(formatBibtexAuthor)
    .filter(Boolean)
    .join(' and ');
  const year = extractYear(paper.publicationDate);
  const fields: string[] = [`  title     = {${paper.title}}`];
  if (author) fields.push(`  author    = {${author}}`);
  if (paper.venue) fields.push(`  journal   = {${paper.venue}}`);
  if (year) fields.push(`  year      = {${year}}`);
  if (paper.url) fields.push(`  url       = {${paper.url}}`);
  return `@article{${key},\n${fields.join(',\n')}\n}`;
}

// --- EndNote (tagged) ---

function toEndnote(paper: Paper): string {
  const year = extractYear(paper.publicationDate);
  const lines: string[] = ['%0 Journal Article', `%T ${paper.title}`];
  paper.authors
    .map((a) => a.trim())
    .filter(Boolean)
    .forEach((a) => lines.push(`%A ${a}`));
  if (paper.venue) lines.push(`%J ${paper.venue}`);
  if (year) lines.push(`%D ${year}`);
  if (paper.url) lines.push(`%U ${paper.url}`);
  return lines.join('\n');
}

// --- MLA (8th ed.) ---

/** MLA author list: first author inverted ("Last, First"); 3+ becomes "et al." */
function formatMlaAuthors(authors: string[]): string {
  const list = authors.map((a) => a.trim()).filter(Boolean);
  if (list.length === 0) return '';
  const formatFirst = (name: string): string => {
    const { last, given } = splitName(name);
    return given.length ? `${last}, ${given.join(' ')}` : last;
  };
  if (list.length === 1) return formatFirst(list[0]);
  if (list.length === 2) {
    const second = splitName(list[1]);
    const secondStr = second.given.length
      ? `${second.given.join(' ')} ${second.last}`
      : second.last;
    return `${formatFirst(list[0])}, and ${secondStr}`;
  }
  return `${formatFirst(list[0])}, et al.`;
}

function toMla(paper: Paper): string {
  const authors = formatMlaAuthors(paper.authors);
  const year = extractYear(paper.publicationDate);
  const parts: string[] = [];
  if (authors) parts.push(ensurePeriod(authors));
  parts.push(`"${paper.title}."`);
  if (paper.venue) parts.push(ensurePeriod(paper.venue));
  if (year) parts.push(`${year}.`);
  if (paper.url) parts.push(`${paper.url}.`);
  return parts.join(' ');
}

// --- Chicago (notes & bibliography) ---

function formatChicagoAuthors(authors: string[]): string {
  const list = authors.map((a) => a.trim()).filter(Boolean);
  if (list.length === 0) return '';
  const formatFirst = (name: string): string => {
    const { last, given } = splitName(name);
    return given.length ? `${last}, ${given.join(' ')}` : last;
  };
  if (list.length === 1) return formatFirst(list[0]);
  if (list.length === 2) {
    const second = splitName(list[1]);
    const secondStr = second.given.length
      ? `${second.given.join(' ')} ${second.last}`
      : second.last;
    return `${formatFirst(list[0])}, and ${secondStr}`;
  }
  return `${formatFirst(list[0])}, et al.`;
}

function toChicago(paper: Paper): string {
  const authors = formatChicagoAuthors(paper.authors);
  const year = extractYear(paper.publicationDate);
  const parts: string[] = [];
  if (authors) parts.push(ensurePeriod(authors));
  parts.push(`"${paper.title}."`);
  if (paper.venue) {
    parts.push(year ? `${paper.venue} (${year}).` : ensurePeriod(paper.venue));
  } else if (year) {
    parts.push(`${year}.`);
  }
  if (paper.url) parts.push(`${paper.url}.`);
  return parts.join(' ');
}

/** Dispatches to the appropriate formatter for the requested citation style. */
export function formatCitation(
  paper: Paper,
  format: Citation['format'],
): string {
  switch (format) {
    case 'apa':
      return toApa(paper);
    case 'bibtex':
      return toBibtex(paper);
    case 'endnote':
      return toEndnote(paper);
    case 'mla':
      return toMla(paper);
    case 'chicago':
      return toChicago(paper);
    default:
      return toApa(paper);
  }
}

// ---------------------------------------------------------------------------
// Routes
// Note: `/export` is registered before `/:paperId` so it is not shadowed.
// ---------------------------------------------------------------------------

/**
 * GET /citations/export
 * Query: format, paperIds (comma-separated). Downloads a concatenated file of
 * citations for the given papers in the requested format.
 */
citationsRouter.get(
  '/export',
  asyncHandler(async (req, res) => {
    const format = req.query.format;
    const paperIdsParam = req.query.paperIds;
    if (!isValidFormat(format)) {
      throw badRequest(
        '"format" query parameter must be one of: bibtex, endnote, apa, mla, chicago',
      );
    }
    if (typeof paperIdsParam !== 'string' || !paperIdsParam.trim()) {
      throw badRequest(
        '"paperIds" query parameter is required and must be a comma-separated list',
      );
    }
    const paperIds = paperIdsParam
      .split(',')
      .map((s) => s.trim())
      .filter(Boolean);
    if (paperIds.length === 0) {
      throw badRequest('"paperIds" must contain at least one id');
    }

    const entries: string[] = [];
    for (const id of paperIds) {
      const paper = getPaperById(id);
      if (paper) {
        entries.push(formatCitation(paper, format));
      }
    }

    const content = entries.join('\n\n');
    const isBibtex = format === 'bibtex';
    res.setHeader(
      'Content-Type',
      isBibtex ? 'application/x-bibtex' : 'text/plain',
    );
    res.setHeader(
      'Content-Disposition',
      `attachment; filename="${isBibtex ? 'citations.bib' : 'citations.txt'}"`,
    );
    res.send(content);
  }),
);

/**
 * GET /citations/:paperId
 * Returns all stored citations for a paper.
 */
citationsRouter.get(
  '/:paperId',
  asyncHandler(async (req, res) => {
    const citations = getCitationsByPaper(req.params.paperId);
    res.json({ citations });
  }),
);

/**
 * POST /citations
 * Body: { paperId, format }. Generates a citation string in the requested
 * format from the paper's data, persists it, and returns it.
 */
citationsRouter.post(
  '/',
  asyncHandler(async (req, res) => {
    const { paperId, format } = req.body ?? {};
    if (typeof paperId !== 'string' || !paperId.trim()) {
      throw badRequest(
        '"paperId" is required and must be a non-empty string',
      );
    }
    if (!isValidFormat(format)) {
      throw badRequest(
        '"format" is required and must be one of: bibtex, endnote, apa, mla, chicago',
      );
    }

    const paper = getPaperById(paperId);
    if (!paper) {
      throw notFound('Paper not found', { id: paperId });
    }

    const content = formatCitation(paper, format);
    const citation = createCitation(paperId, format, content);
    res.status(201).json({ citation });
  }),
);
