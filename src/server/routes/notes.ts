import express from 'express';
import type { Note } from '../../shared/types';
import {
  createNote,
  deleteNote,
  getNoteById,
  getNotes,
  updateNote,
} from '../database/queries';
import { asyncHandler, badRequest, notFound } from '../utils/httpError';

export const notesRouter = express.Router();

/**
 * GET /notes
 * Query: paperId (optional). Returns notes, optionally filtered by paper.
 */
notesRouter.get(
  '/',
  asyncHandler(async (req, res) => {
    const paperId =
      typeof req.query.paperId === 'string' ? req.query.paperId : undefined;
    const notes = getNotes(paperId);
    res.json({ notes });
  }),
);

/**
 * GET /notes/:id
 */
notesRouter.get(
  '/:id',
  asyncHandler(async (req, res) => {
    const note = getNoteById(req.params.id);
    if (!note) {
      throw notFound('Note not found', { id: req.params.id });
    }
    res.json({ note });
  }),
);

/**
 * POST /notes
 * Body: { title, content, tags?: string[], paperId?: string }
 */
notesRouter.post(
  '/',
  asyncHandler(async (req, res) => {
    const { title, content, tags, paperId } = req.body ?? {};

    if (typeof title !== 'string' || !title.trim()) {
      throw badRequest('"title" is required and must be a non-empty string');
    }
    if (typeof content !== 'string') {
      throw badRequest('"content" is required and must be a string');
    }
    if (tags !== undefined && !Array.isArray(tags)) {
      throw badRequest('"tags" must be an array of strings');
    }
    if (
      paperId !== undefined &&
      (typeof paperId !== 'string' || !paperId.trim())
    ) {
      throw badRequest('"paperId" must be a non-empty string when provided');
    }

    const noteInput: Omit<Note, 'id' | 'createdAt' | 'updatedAt'> = {
      title: title.trim(),
      content,
      tags: Array.isArray(tags) ? tags : [],
      paperId: typeof paperId === 'string' ? paperId : undefined,
    };

    const note = createNote(noteInput);
    res.status(201).json({ note });
  }),
);

/**
 * PUT /notes/:id
 * Body: Partial<Note>
 */
notesRouter.put(
  '/:id',
  asyncHandler(async (req, res) => {
    const data = (req.body ?? {}) as Partial<Note>;
    if (
      data.tags !== undefined &&
      (!Array.isArray(data.tags) ||
        data.tags.some((t) => typeof t !== 'string'))
    ) {
      throw badRequest('"tags" must be an array of strings');
    }
    const updated = updateNote(req.params.id, data);
    if (!updated) {
      throw notFound('Note not found', { id: req.params.id });
    }
    res.json({ note: updated });
  }),
);

/**
 * DELETE /notes/:id
 */
notesRouter.delete(
  '/:id',
  asyncHandler(async (req, res) => {
    const success = deleteNote(req.params.id);
    if (!success) {
      throw notFound('Note not found', { id: req.params.id });
    }
    res.json({ success: true });
  }),
);
