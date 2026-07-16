import type { Application } from 'express';
import { aiRouter } from './ai';
import { citationsRouter } from './citations';
import { graphRouter } from './graph';
import { notesRouter } from './notes';
import { papersRouter } from './papers';
import { projectsRouter } from './projects';
import { searchRouter } from './search';

export {
  aiRouter,
  citationsRouter,
  graphRouter,
  notesRouter,
  papersRouter,
  projectsRouter,
  searchRouter,
};

/**
 * Mounts every API router onto the Express application under `/api`.
 * Must be called after body-parsing middleware and before the error-handling
 * middleware.
 */
export function mountRoutes(app: Application): void {
  app.use('/api/papers', papersRouter);
  app.use('/api/search', searchRouter);
  app.use('/api/notes', notesRouter);
  app.use('/api/projects', projectsRouter);
  app.use('/api/ai', aiRouter);
  app.use('/api/citations', citationsRouter);
  app.use('/api/graph', graphRouter);
}
