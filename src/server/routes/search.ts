import express from 'express';
import type { SearchFilters } from '../../shared/types';
import {
  createSavedSearch,
  deleteSavedSearch,
  getSavedSearches,
  searchPapers,
} from '../database/queries';
import {
  asyncHandler,
  badRequest,
  notFound,
  parsePositiveInt,
} from '../utils/httpError';

export const searchRouter = express.Router();

/**
 * Normalizes and validates a SearchFilters payload coming from the client.
 * Returns a clean SearchFilters object (empty when no filters were provided).
 */
function normalizeFilters(raw: unknown): SearchFilters {
  if (raw === null || raw === undefined) return {};
  if (typeof raw !== 'object' || Array.isArray(raw)) {
    throw badRequest('"filters" must be an object');
  }
  const f = raw as Record<string, unknown>;
  const filters: SearchFilters = {};

  if (f.dateRange !== undefined) {
    if (
      typeof f.dateRange !== 'object' ||
      f.dateRange === null ||
      Array.isArray(f.dateRange)
    ) {
      throw badRequest('"filters.dateRange" must be an object');
    }
    const dr = f.dateRange as { start?: unknown; end?: unknown };
    if (
      (dr.start !== undefined && typeof dr.start !== 'string') ||
      (dr.end !== undefined && typeof dr.end !== 'string')
    ) {
      throw badRequest('"filters.dateRange.start" and ".end" must be strings');
    }
    filters.dateRange = {
      start: typeof dr.start === 'string' ? dr.start : '',
      end: typeof dr.end === 'string' ? dr.end : '',
    };
  }

  if (f.venues !== undefined) {
    if (!Array.isArray(f.venues) || f.venues.some((v) => typeof v !== 'string')) {
      throw badRequest('"filters.venues" must be an array of strings');
    }
    filters.venues = f.venues as string[];
  }

  if (f.authors !== undefined) {
    if (!Array.isArray(f.authors) || f.authors.some((a) => typeof a !== 'string')) {
      throw badRequest('"filters.authors" must be an array of strings');
    }
    filters.authors = f.authors as string[];
  }

  if (f.keywords !== undefined) {
    if (!Array.isArray(f.keywords) || f.keywords.some((k) => typeof k !== 'string')) {
      throw badRequest('"filters.keywords" must be an array of strings');
    }
    filters.keywords = f.keywords as string[];
  }

  return filters;
}

/**
 * POST /search
 * Body: { query: string, filters?: SearchFilters, page?, limit? }
 * Performs a real search against the local papers table.
 */
searchRouter.post(
  '/',
  asyncHandler(async (req, res) => {
    const { query } = req.body ?? {};
    if (typeof query !== 'string') {
      throw badRequest('"query" is required and must be a string');
    }
    const filters = normalizeFilters(req.body?.filters);
    const page = parsePositiveInt(req.body?.page, 1);
    const limit = parsePositiveInt(req.body?.limit, 20);

    const { papers, total } = searchPapers(query, filters, page, limit);
    res.json({ results: papers, total, sources: ['local'] });
  }),
);

/**
 * GET /search/saved
 * Returns all saved search queries.
 */
searchRouter.get(
  '/saved',
  asyncHandler(async (_req, res) => {
    const queries = getSavedSearches();
    res.json({ queries });
  }),
);

/**
 * POST /search/saved
 * Body: { query: string, filters?: SearchFilters }
 */
searchRouter.post(
  '/saved',
  asyncHandler(async (req, res) => {
    const { query } = req.body ?? {};
    if (typeof query !== 'string' || !query.trim()) {
      throw badRequest('"query" is required and must be a non-empty string');
    }
    const filters = normalizeFilters(req.body?.filters);
    const saved = createSavedSearch(query, filters);
    res.status(201).json({ query: saved });
  }),
);

/**
 * DELETE /search/saved/:id
 */
searchRouter.delete(
  '/saved/:id',
  asyncHandler(async (req, res) => {
    const success = deleteSavedSearch(req.params.id);
    if (!success) {
      throw notFound('Saved search not found', { id: req.params.id });
    }
    res.json({ success: true });
  }),
);
