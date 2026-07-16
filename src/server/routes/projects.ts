import express from 'express';
import type { Note, Paper, Project } from '../../shared/types';
import {
  addProjectMember,
  createProject,
  deleteProject,
  deleteProjectMembers,
  getProjectById,
  getProjectMembers,
  getProjects,
  removeProjectMember,
  updateProject,
} from '../database/queries';
import { asyncHandler, badRequest, notFound } from '../utils/httpError';

export const projectsRouter = express.Router();

/**
 * GET /projects
 */
projectsRouter.get(
  '/',
  asyncHandler(async (_req, res) => {
    const projects = getProjects();
    res.json({ projects });
  }),
);

/**
 * GET /projects/:id
 * Returns the project, its members, and any linked papers/notes.
 */
projectsRouter.get(
  '/:id',
  asyncHandler(async (req, res) => {
    const project = getProjectById(req.params.id);
    if (!project) {
      throw notFound('Project not found', { id: req.params.id });
    }
    const members = getProjectMembers(req.params.id);
    // TODO: depends on project-paper linking
    // Papers and notes are not yet linked to projects in the schema, so these
    // are empty until a project_papers relationship is introduced.
    const papers: Paper[] = [];
    const notes: Note[] = [];
    res.json({ project, members, papers, notes });
  }),
);

/**
 * POST /projects
 * Body: { name, description }
 */
projectsRouter.post(
  '/',
  asyncHandler(async (req, res) => {
    const { name, description } = req.body ?? {};
    if (typeof name !== 'string' || !name.trim()) {
      throw badRequest('"name" is required and must be a non-empty string');
    }
    if (description !== undefined && typeof description !== 'string') {
      throw badRequest('"description" must be a string when provided');
    }
    const project = createProject({
      name: name.trim(),
      description: typeof description === 'string' ? description : '',
      members: [],
    });
    res.status(201).json({ project });
  }),
);

/**
 * PUT /projects/:id
 * Body: Partial<Project>
 */
projectsRouter.put(
  '/:id',
  asyncHandler(async (req, res) => {
    const data = (req.body ?? {}) as Partial<Project>;
    if (
      data.name !== undefined &&
      (typeof data.name !== 'string' || !data.name.trim())
    ) {
      throw badRequest('"name" must be a non-empty string when provided');
    }
    if (data.description !== undefined && typeof data.description !== 'string') {
      throw badRequest('"description" must be a string when provided');
    }
    const updated = updateProject(req.params.id, data);
    if (!updated) {
      throw notFound('Project not found', { id: req.params.id });
    }
    res.json({ project: updated });
  }),
);

/**
 * DELETE /projects/:id
 * Also removes all project_members rows for this project.
 */
projectsRouter.delete(
  '/:id',
  asyncHandler(async (req, res) => {
    const existing = getProjectById(req.params.id);
    if (!existing) {
      throw notFound('Project not found', { id: req.params.id });
    }
    deleteProjectMembers(req.params.id);
    deleteProject(req.params.id);
    res.json({ success: true });
  }),
);

/**
 * POST /projects/:id/members
 * Body: { userId: string }
 */
projectsRouter.post(
  '/:id/members',
  asyncHandler(async (req, res) => {
    const project = getProjectById(req.params.id);
    if (!project) {
      throw notFound('Project not found', { id: req.params.id });
    }
    const { userId } = req.body ?? {};
    if (typeof userId !== 'string' || !userId.trim()) {
      throw badRequest('"userId" is required and must be a non-empty string');
    }
    addProjectMember(req.params.id, userId.trim());
    res.status(201).json({ success: true });
  }),
);

/**
 * DELETE /projects/:id/members/:userId
 */
projectsRouter.delete(
  '/:id/members/:userId',
  asyncHandler(async (req, res) => {
    const success = removeProjectMember(req.params.id, req.params.userId);
    if (!success) {
      throw notFound('Project member not found', {
        projectId: req.params.id,
        userId: req.params.userId,
      });
    }
    res.json({ success: true });
  }),
);
