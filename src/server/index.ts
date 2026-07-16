import * as http from 'http';
import express from 'express';
// Default import is used (instead of `import { WebSocketServer }`) because `ws`
// attaches `WebSocketServer` to its CJS export indirectly, which Node's ESM
// named-export detection cannot see under `"type": "module"` builds.
import WebSocket from 'ws';
// Importing the database module initializes the singleton (opens the SQLite
// file and creates tables) so it is ready before serving any requests.
import './database';

const app = express();

// CORS: allow all origins for local development (no external dependency).
app.use((req, res, next) => {
  res.header('Access-Control-Allow-Origin', '*');
  res.header(
    'Access-Control-Allow-Methods',
    'GET, POST, PUT, DELETE, PATCH, OPTIONS',
  );
  res.header('Access-Control-Allow-Headers', 'Content-Type, Authorization');
  if (req.method === 'OPTIONS') {
    res.sendStatus(204);
    return;
  }
  next();
});

app.use(express.json());

// Health check endpoint.
app.get('/api/health', (_req, res) => {
  res.json({ status: 'ok', version: '1.0.0' });
});

// TODO: depends on agent-02 implementing route handlers in ./routes/

const server = http.createServer(app);

// WebSocket server for real-time collaboration, mounted on the same HTTP
// server at /ws.
const wss = new WebSocket.WebSocketServer({ server, path: '/ws' });

wss.on('connection', () => {
  // Realtime collaboration handlers will be implemented in src/server/realtime/.
});

// Error handling middleware (must be registered after all routes).
const errorHandler: express.ErrorRequestHandler = (err, _req, res, _next) => {
  const statusCode = err?.status ?? 500;
  const code = err?.code ?? 'INTERNAL_ERROR';
  const message = err?.message ?? 'Internal server error';
  res.status(statusCode).json({
    error: { code, message, details: err?.details },
  });
};
app.use(errorHandler);

server.listen(3000, () => {
  console.log('Open Research server listening on http://localhost:3000');
});

export default server;
