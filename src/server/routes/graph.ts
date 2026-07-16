import express from 'express';
import { asyncHandler, badRequest, notImplemented } from '../utils/httpError';

export const graphRouter = express.Router();

/**
 * GET /graph/:projectId
 * Returns the knowledge graph (nodes + edges) for a project.
 */
graphRouter.get(
  '/:projectId',
  asyncHandler(async (_req, res) => {
    // TODO: depends on entity-extraction/graph-build service
    // No graph data is produced yet because entity extraction and graph
    // construction are not implemented. Return the correct empty shape so the
    // client can render an empty graph.
    res.json({ nodes: [], edges: [] });
  }),
);

/**
 * POST /graph/build
 * Body: { projectId: string }
 */
graphRouter.post(
  '/build',
  asyncHandler(async (req, res) => {
    const { projectId } = req.body ?? {};
    if (typeof projectId !== 'string' || !projectId.trim()) {
      throw badRequest(
        '"projectId" is required and must be a non-empty string',
      );
    }
    throw notImplemented(
      'Graph building requires entity extraction (future cycle)',
    );
  }),
);
