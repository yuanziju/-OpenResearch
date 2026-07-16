# Open Research - Project Specification

## 1. Project Overview

### 1.1 Mission
Open Research is an open-source research tool that redefines the research workflow. Built before Anthropic's official product definition, it takes inspiration from Claude Science while introducing significant improvements and innovations.

### 1.2 Core Value Proposition
- **Pioneering Concept**: Defines the "research assistant" category before industry leaders
- **Open-Source**: Community-driven development with full transparency
- **Superior Experience**: Built with modern technologies and user-centric design
- **Extensible**: Modular architecture allowing easy integration of new features

## 2. Core Features

### 2.1 Advanced Paper Search
- Semantic search across academic databases
- Citation-based relevance ranking
- Filter by publication date, venue, authors, keywords
- Save search queries and set up alerts
- Cross-database unified search (arXiv, PubMed, Google Scholar, Semantic Scholar)

### 2.2 Real-time Collaboration
- Multi-user simultaneous editing
- Real-time cursor tracking
- Comment system with mentions
- Version history with diff view
- Shared workspaces and projects

### 2.3 AI-powered Analysis
- Automatic paper summarization
- Key insight extraction
- Related papers recommendation
- Literature review generation
- Citation network analysis
- Research trend identification

### 2.4 Data Integration
- Unified access to multiple academic databases
- Local file import (PDF, DOCX, etc.)
- Integration with reference managers (Zotero, Mendeley)
- API for custom data sources
- Data normalization and deduplication

### 2.5 Knowledge Graph
- Visual representation of research concepts
- Automatic entity extraction
- Relationship mapping between papers
- Interactive graph exploration
- Export to standard formats

### 2.6 Citation Management
- BibTeX/EndNote export
- Citation style support (APA, MLA, Chicago)
- Automatic citation generation
- Citation count tracking
- Reference list management

### 2.7 Research Notes
- Rich text editor with markdown support
- Note linking and organization
- Tagging system
- Search across notes
- Export options

## 3. Technical Architecture

### 3.1 Tech Stack
- **Language**: TypeScript (Full-stack)
- **Frontend**: Open Designer
- **Backend**: TypeScript (Node.js/Deno)
- **Database**: SQLite (local) + IndexedDB (client-side caching)
- **Real-time**: WebSockets or Server-Sent Events
- **AI Integration**: OpenAI API, Anthropic API, or local models

### 3.2 System Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                      Client (Browser)                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ UI Layer    │  │ State Mgmt  │  │  API Client         │  │
│  │ Open Designer│  │ Zustand     │  │  WebSocket Client   │  │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
│         │                │                    │             │
│  ┌──────▼────────────────▼────────────────────▼──────────┐  │
│  │              Shared Types & Utilities                 │  │
│  └───────────────────────────────────────────────────────┘  │
└───────────────────────────────────────┬─────────────────────┘
                                        │ HTTP/WebSocket
┌───────────────────────────────────────▼─────────────────────┐
│                     Server (Local)                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ API Routes  │  │  Services   │  │  AI Integration     │  │
│  │ REST + WS   │  │  Business   │  │  External APIs      │  │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
│         │                │                    │             │
│  ┌──────▼────────────────▼────────────────────▼──────────┐  │
│  │              Database Layer (SQLite)                   │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### 3.3 Project Structure
```
/workspace/
├── src/
│   ├── client/
│   │   ├── components/          # UI components
│   │   ├── pages/               # Page views
│   │   ├── state/               # State management
│   │   ├── api/                 # API clients
│   │   ├── utils/               # Client utilities
│   │   └── index.tsx            # Entry point
│   ├── server/
│   │   ├── routes/              # API routes
│   │   ├── services/            # Business logic
│   │   ├── database/            # Database models and queries
│   │   ├── ai/                  # AI integrations
│   │   ├── realtime/            # WebSocket handlers
│   │   └── index.ts             # Server entry point
│   └── shared/
│       ├── types/               # TypeScript type definitions
│       ├── constants/           # Global constants
│       └── utils/               # Shared utilities
├── .gitignore
├── AGENTS.md
├── README.md
├── spec.md
└── package.json
```

## 4. Development Workflow

### 4.1 Branch Management
- Lead agent creates `dev_{hash}` branches for each development cycle
- Sub-agents work on their dedicated branches
- Sub-agents commit and push to their branches
- Lead agent handles merge back to main
- Reuse intact branches, discard corrupted ones

### 4.2 Decision Making
- Technical decisions made by lead agent
- Major decisions require human review via `a uq`
- All decisions documented in spec.md

### 4.3 Code Quality
- TypeScript strict mode enabled
- ESLint for code linting
- Prettier for code formatting
- Unit tests for critical logic

## 5. Deployment

### 5.1 Environment
- **Deployment**: Local application
- **Database**: SQLite file storage
- **Network**: No external server dependency required

### 5.2 Installation
```bash
npm install
npm run dev
```

### 5.3 Production Build
```bash
npm run build
npm run start
```

## 6. Key Decisions

### 6.1 Architecture Pattern
- **Decision**: Monorepo with client-server separation
- **Rationale**: Simplifies shared types, improves maintainability
- **Status**: Proposed

### 6.2 State Management
- **Decision**: Zustand for client state
- **Rationale**: Lightweight, TypeScript-friendly, minimal boilerplate
- **Status**: Proposed

### 6.3 Database
- **Decision**: SQLite for local storage
- **Rationale**: Zero-config, file-based, suitable for local app
- **Status**: Proposed

### 6.4 Real-time Communication
- **Decision**: WebSockets for collaboration
- **Rationale**: Full-duplex communication, real-time updates
- **Status**: Proposed

## 7. Future Enhancements

- Mobile app support
- Plugin system for custom integrations
- Offline mode
- Advanced visualization tools
- Machine learning models for better recommendations
