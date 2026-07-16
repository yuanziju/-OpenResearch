import Database from 'better-sqlite3'
import { DATABASE_NAME } from '@/shared/constants'
import path from 'path'

export default class ResearchDatabase {
  private db: Database.Database

  constructor() {
    const dbPath = path.join(process.cwd(), DATABASE_NAME)
    this.db = new Database(dbPath)
    this.initialize()
  }

  private initialize() {
    this.db.exec(`
      CREATE TABLE IF NOT EXISTS papers (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        authors TEXT NOT NULL,
        abstract TEXT,
        publication_date TEXT,
        venue TEXT,
        citations INTEGER DEFAULT 0,
        url TEXT,
        pdf_url TEXT,
        keywords TEXT,
        references TEXT
      );

      CREATE TABLE IF NOT EXISTS notes (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        content TEXT,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        tags TEXT,
        paper_id TEXT,
        FOREIGN KEY (paper_id) REFERENCES papers(id)
      );

      CREATE TABLE IF NOT EXISTS projects (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
      );

      CREATE TABLE IF NOT EXISTS project_members (
        project_id TEXT,
        user_id TEXT,
        PRIMARY KEY (project_id, user_id),
        FOREIGN KEY (project_id) REFERENCES projects(id)
      );

      CREATE TABLE IF NOT EXISTS search_queries (
        id TEXT PRIMARY KEY,
        query TEXT NOT NULL,
        filters TEXT,
        created_at TEXT NOT NULL,
        saved INTEGER DEFAULT 0
      );

      CREATE TABLE IF NOT EXISTS citations (
        id TEXT PRIMARY KEY,
        paper_id TEXT NOT NULL,
        format TEXT NOT NULL,
        content TEXT NOT NULL,
        FOREIGN KEY (paper_id) REFERENCES papers(id)
      );
    `)
  }

  public close() {
    this.db.close()
  }
}
