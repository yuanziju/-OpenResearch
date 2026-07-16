# Open Research - Agent Workflow

## 1. Workflow Rules

### 1.1 Branch Management

- Each sub-agent has a dedicated branch named `agent-{NN}` (e.g., `agent-01`, `agent-02`)
- Sub-agent branches are NOT disposable. They are tied to the sub-agent ID and reused across development cycles
- If a sub-agent's branch becomes corrupted, discard it and create a new branch with the same name (e.g., delete `agent-01` and recreate `agent-01`)
- For each development cycle, the lead agent creates a new `dev_{hash}` branch from `main`
- Sub-agents commit to their own branches
- The lead agent merges sub-agent branches into `dev_{hash}`
- After all sub-agent tasks are complete, the lead agent merges `dev_{hash}` back to `main`

### 1.2 Merge Flow

```
main ──> dev_{hash} ──> agent-01 (sub-agent #1 works here)
                  ──> agent-02 (sub-agent #2 works here)
                  ──> agent-03 (sub-agent #3 works here)

After all sub-agents finish:
agent-01 ──> dev_{hash}
agent-02 ──> dev_{hash}
agent-03 ──> dev_{hash}
dev_{hash} ──> main
```

### 1.3 Tool Abbreviations

- `a uq` = Ask User Question tool

### 1.4 Decision Making

- Lead agent has high autonomy in technical decisions
- Major decisions require human review via `a uq`
- All decisions documented in spec.md
- Lead agent defines API contracts and requirements before dispatching sub-agent tasks

### 1.5 Communication

- Use clear, concise language
- Document all significant decisions in spec.md
- Keep user informed of progress

## 2. Sub-Agent System

### 2.1 Sub-Agent Registry

| ID | Name | Branch | Role |
|----|------|--------|------|
| #1 | backend-core | agent-01 | Server entry point, database setup, health check, basic middleware |
| #2 | backend-api | agent-02 | REST API route handlers, business logic services |
| #3 | frontend-core | agent-03 | React app skeleton, layout, routing, Open Designer integration |
| #4 | frontend-state | agent-04 | API client, Zustand store, WebSocket client |

### 2.2 Sub-Agent Guidelines

- Sub-agents receive detailed task prompts from the lead agent
- Task prompts include: API contract references, data formats, file paths, and acceptance criteria
- Sub-agents can add TODO comments for unimplemented features that depend on other sub-agents' work
- TODO comments must include the dependency: `// TODO: depends on agent-XX implementing YYY`
- Sub-agents should NOT guess how other modules work. Reference spec.md section 7 (API Contract) for interface definitions
- Sub-agents must NOT create files outside their assigned scope without lead agent approval
- Sub-agents should avoid "lazy implementation" patterns:
  - Do not return hardcoded mock data without a TODO comment
  - Do not skip error handling with "this is complex" excuses
  - Do not leave empty function bodies without explanation

## 3. Project Structure

```
/workspace/
├── .github/
│   └── workflows/
│       └── ci.yml              # GitHub Actions CI
├── src/
│   ├── client/                 # Frontend (React + TypeScript)
│   │   ├── components/         # UI components
│   │   ├── pages/              # Page views
│   │   ├── state/              # Zustand state management
│   │   ├── api/                # API client and WebSocket client
│   │   └── utils/              # Client utilities
│   ├── server/                 # Backend (TypeScript)
│   │   ├── routes/             # REST API route handlers
│   │   ├── services/           # Business logic
│   │   ├── database/           # SQLite database setup and queries
│   │   ├── ai/                 # AI integration services
│   │   └── realtime/           # WebSocket handlers
│   └── shared/                 # Shared between client and server
│       ├── types/              # TypeScript type definitions
│       ├── constants/          # Global constants
│       └── utils/              # Shared utilities
├── .gitignore
├── AGENTS.md
├── README.md
├── spec.md
├── package.json
├── tsconfig.json
├── tsconfig.server.json
└── vite.config.ts
```

## 4. Technical Stack

- **Language**: TypeScript (Full-stack)
- **Frontend**: React with Open Designer, Vite build tool
- **Backend**: Node.js HTTP server, WebSockets
- **Database**: SQLite (better-sqlite3)
- **State Management**: Zustand
- **Desktop**: Electron
- **Deployment**: Local

## 5. API Contract Reference

See spec.md section 7 for the complete API contract:
- Section 7.1: REST API endpoints (Papers, Search, Notes, Projects, AI, Citations, Graph, Health)
- Section 7.2: WebSocket protocol (join, leave, cursor, note_edit, comment)
- Section 7.3: Error response format

All sub-agents MUST follow the API contract defined in spec.md. Do not invent new endpoints or change response formats without lead agent approval.
