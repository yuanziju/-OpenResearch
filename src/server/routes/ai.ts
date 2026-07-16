// TODO: depends on AI provider integration (future cycle)
// The handlers below are fully wired and validate their inputs, but the
// actual AI work requires an external AI provider that is not yet integrated.
// Each returns 501 NOT_IMPLEMENTED until that integration lands.
import express from 'express';
import { asyncHandler, badRequest, notImplemented } from '../utils/httpError';

export const aiRouter = express.Router();

/**
 * POST /ai/summarize
 * Body: { paperId: string }
 */
aiRouter.post(
  '/summarize',
  asyncHandler(async (req, res) => {
    const { paperId } = req.body ?? {};
    if (typeof paperId !== 'string' || !paperId.trim()) {
      throw badRequest('"paperId" is required and must be a non-empty string');
    }
    throw notImplemented(
      'AI summarization requires an AI provider integration (future cycle)',
    );
  }),
);

/**
 * POST /ai/insights
 * Body: { paperId: string }
 */
aiRouter.post(
  '/insights',
  asyncHandler(async (req, res) => {
    const { paperId } = req.body ?? {};
    if (typeof paperId !== 'string' || !paperId.trim()) {
      throw badRequest('"paperId" is required and must be a non-empty string');
    }
    throw notImplemented(
      'AI insights require an AI provider integration (future cycle)',
    );
  }),
);

/**
 * POST /ai/related
 * Body: { paperId: string, limit?: number }
 */
aiRouter.post(
  '/related',
  asyncHandler(async (req, res) => {
    const { paperId, limit } = req.body ?? {};
    if (typeof paperId !== 'string' || !paperId.trim()) {
      throw badRequest('"paperId" is required and must be a non-empty string');
    }
    if (
      limit !== undefined &&
      (typeof limit !== 'number' || !Number.isFinite(limit) || limit < 1)
    ) {
      throw badRequest('"limit" must be a positive number when provided');
    }
    throw notImplemented(
      'AI related-paper discovery requires an AI provider integration (future cycle)',
    );
  }),
);

/**
 * POST /ai/review
 * Body: { paperIds: string[], topic?: string }
 */
aiRouter.post(
  '/review',
  asyncHandler(async (req, res) => {
    const { paperIds, topic } = req.body ?? {};
    if (
      !Array.isArray(paperIds) ||
      paperIds.length === 0 ||
      paperIds.some((id) => typeof id !== 'string' || !id.trim())
    ) {
      throw badRequest(
        '"paperIds" is required and must be a non-empty array of non-empty strings',
      );
    }
    if (topic !== undefined && typeof topic !== 'string') {
      throw badRequest('"topic" must be a string when provided');
    }
    throw notImplemented(
      'AI literature review requires an AI provider integration (future cycle)',
    );
  }),
);

/**
 * POST /ai/trends
 * Body: { topic: string, timeframe?: string }
 */
aiRouter.post(
  '/trends',
  asyncHandler(async (req, res) => {
    const { topic, timeframe } = req.body ?? {};
    if (typeof topic !== 'string' || !topic.trim()) {
      throw badRequest('"topic" is required and must be a non-empty string');
    }
    if (timeframe !== undefined && typeof timeframe !== 'string') {
      throw badRequest('"timeframe" must be a string when provided');
    }
    throw notImplemented(
      'AI trend analysis requires an AI provider integration (future cycle)',
    );
  }),
);
