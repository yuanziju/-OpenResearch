import express from 'express';
import type { Paper } from '../../shared/types';
import {
  createPaper,
  deletePaper,
  getPaperById,
  getPapersPaged,
  updatePaper,
} from '../database/queries';
import {
  asyncHandler,
  badRequest,
  notFound,
  notImplemented,
  parsePositiveInt,
} from '../utils/httpError';

export const papersRouter = express.Router();

/**
 * GET /papers
 * Query: page, limit, sort, order. Returns a paginated list of papers.
 */
papersRouter.get(
  '/',
  asyncHandler(async (req, res) => {
    const page = parsePositiveInt(req.query.page, 1);
    const limit = parsePositiveInt(req.query.limit, 20);
    const sort =
      typeof req.query.sort === 'string' && req.query.sort.length > 0
        ? req.query.sort
        : 'publicationDate';
    const order = req.query.order === 'asc' ? 'asc' : 'desc';

    const { papers, total } = getPapersPaged(page, limit, sort, order);
    res.json({ papers, total, page, limit });
  }),
);

/**
 * GET /papers/:id
 * Returns a single paper by id.
 */
papersRouter.get(
  '/:id',
  asyncHandler(async (req, res) => {
    const paper = getPaperById(req.params.id);
    if (!paper) {
      throw notFound('Paper not found', { id: req.params.id });
    }
    res.json({ paper });
  }),
);

/**
 * POST /papers
 * Body: { source: 'url'|'file'|'manual', url?, file?(FormData), data? }
 * - 'manual': create a paper from `data`.
 * - 'url' / 'file': external import is not yet implemented (501).
 */
papersRouter.post(
  '/',
  asyncHandler(async (req, res) => {
    const { source } = req.body ?? {};
    if (source !== 'url' && source !== 'file' && source !== 'manual') {
      throw badRequest('Invalid source; expected "url", "file", or "manual"', {
        source,
      });
    }

    if (source === 'url' || source === 'file') {
      throw notImplemented(
        'Paper import from url/file is not yet implemented',
        { source },
      );
    }

    // source === 'manual'
    const data = (req.body.data ?? {}) as Partial<Paper>;
    if (!data.title || typeof data.title !== 'string' || !data.title.trim()) {
      throw badRequest('A non-empty "data.title" is required for manual import');
    }
    if (!Array.isArray(data.authors)) {
      throw badRequest('"data.authors" must be an array of strings');
    }

    const paper = createPaper({
      title: data.title.trim(),
      authors: data.authors,
      abstract: typeof data.abstract === 'string' ? data.abstract : '',
      publicationDate:
        typeof data.publicationDate === 'string' ? data.publicationDate : '',
      venue: typeof data.venue === 'string' ? data.venue : '',
      citations: typeof data.citations === 'number' ? data.citations : 0,
      url: typeof data.url === 'string' ? data.url : '',
      pdfUrl: typeof data.pdfUrl === 'string' ? data.pdfUrl : undefined,
      keywords: Array.isArray(data.keywords) ? data.keywords : [],
      references: Array.isArray(data.references) ? data.references : [],
    });

    res.status(201).json({ paper });
  }),
);

/**
 * PUT /papers/:id
 * Body: Partial<Paper>. Updates an existing paper.
 */
papersRouter.put(
  '/:id',
  asyncHandler(async (req, res) => {
    const data = (req.body ?? {}) as Partial<Paper>;
    const updated = updatePaper(req.params.id, data);
    if (!updated) {
      throw notFound('Paper not found', { id: req.params.id });
    }
    res.json({ paper: updated });
  }),
);

/**
 * DELETE /papers/:id
 */
papersRouter.delete(
  '/:id',
  asyncHandler(async (req, res) => {
    const success = deletePaper(req.params.id);
    if (!success) {
      throw notFound('Paper not found', { id: req.params.id });
    }
    res.json({ success: true });
  }),
);
