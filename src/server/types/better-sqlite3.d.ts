// Minimal ambient type declarations for `better-sqlite3`.
// The package does not ship its own types and `@types/better-sqlite3` is not
// installed in this workspace. This declaration exposes only the API surface
// used by the server (constructor, prepare/exec/close, statement run/get/all).

declare module 'better-sqlite3' {
  export interface RunResult {
    changes: number;
    lastInsertRowid: number | bigint;
  }

  export interface Statement<T = unknown> {
    run(...params: unknown[]): RunResult;
    get(...params: unknown[]): T | undefined;
    all(...params: unknown[]): T[];
  }

  export class Database {
    constructor(filename: string, options?: Database.Options);

    prepare<T = unknown>(sql: string): Statement<T>;

    exec(sql: string): void;

    pragma(pragma: string, options?: { simple?: boolean }): unknown;

    close(): void;
  }

  export namespace Database {
    interface Options {
      readonly?: boolean;
      fileMustExist?: boolean;
      timeout?: number;
      verbose?: (...args: unknown[]) => void;
    }
  }

  export default Database;
}
