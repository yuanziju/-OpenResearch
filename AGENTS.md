# Open Research - Agent Workflow

## Workflow Rules

1. **Branch Management**:
   - Each sub-agent has its own dedicated branch
   - Sub-agents commit to their own branches
   - Merge back to main branch is performed by the lead agent
   - Create new `dev_{hash}` branches for each cycle
   - If previous branch is intact, reuse it; if corrupted, discard and create new

2. **Tool Abbreviations**:
   - `a uq` = Ask User Question tool

3. **Decision Making**:
   - Lead agent has high autonomy in technical decisions
   - Major decisions require human review
   - Propose decisions to user for approval

4. **Communication**:
   - Use clear, concise language
   - Document all significant decisions in spec.md
   - Keep user informed of progress

## Project Structure

```
/workspace/
├── .gitignore
├── AGENTS.md
├── README.md
├── spec.md
├── src/
│   ├── client/          # Frontend (Open Designer + TypeScript)
│   ├── server/          # Backend (TypeScript)
│   └── shared/          # Shared types and utilities
└── package.json
```

## Agent Roles

- **Lead Agent**: Overall project coordination, branch management, decision making
- **Sub-agents**: Specialized tasks (frontend, backend, API integration, etc.)

## Technical Stack

- Full-stack TypeScript
- Frontend: Open Designer
- Local deployment
