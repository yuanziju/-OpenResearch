import type { RequestHandler, Request, Response, NextFunction } from 'express';

/**
 * HTTP error carrying a status code, a machine-readable error code, a
 * human-readable message, and optional structured details. The server's
 * error-handling middleware reads `status`, `code`, `message`, and
 * `details` off any thrown error, so throwing an `HttpError` from a route
 * handler produces a correctly-shaped JSON error response.
 */
export class HttpError extends Error {
  constructor(
    public status: number,
    public code: string,
    message: string,
    public details?: unknown,
  ) {
    super(message);
    this.name = 'HttpError';
  }
}

/**
 * Convenience constructors for the most common error codes used across the
 * API. Each returns a ready-to-throw `HttpError`.
 */
export const badRequest = (message: string, details?: unknown): HttpError =>
  new HttpError(400, 'VALIDATION_ERROR', message, details);

export const notFound = (message: string, details?: unknown): HttpError =>
  new HttpError(404, 'NOT_FOUND', message, details);

export const notImplemented = (message: string, details?: unknown): HttpError =>
  new HttpError(501, 'NOT_IMPLEMENTED', message, details);

export const internalError = (message: string, details?: unknown): HttpError =>
  new HttpError(500, 'INTERNAL_ERROR', message, details);

/**
 * Wraps an async route handler so that a rejected promise is forwarded to
 * Express's `next` and reaches the centralized error-handling middleware.
 * Express 4 does not catch rejected promises from handlers automatically,
 * so every async handler must be wrapped (or wrap its body in try/catch).
 */
type AsyncRequestHandler = (
  req: Request,
  res: Response,
  next: NextFunction,
) => Promise<unknown>;

export function asyncHandler(fn: AsyncRequestHandler): RequestHandler {
  return (req, res, next) => {
    Promise.resolve(fn(req, res, next)).catch(next);
  };
}

/**
 * Parses a query-string value into a positive integer, falling back to the
 * provided default when the value is missing or invalid. Used for pagination
 * parameters (`page`, `limit`).
 */
export function parsePositiveInt(
  value: unknown,
  defaultValue: number,
): number {
  if (typeof value !== 'string' && typeof value !== 'number') {
    return defaultValue;
  }
  const parsed = Number.parseInt(String(value), 10);
  if (!Number.isFinite(parsed) || parsed < 1) {
    return defaultValue;
  }
  return parsed;
}
