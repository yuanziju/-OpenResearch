import { WS_BASE_URL } from '@/shared/constants'

export type RealtimeEvent =
  | 'user_joined'
  | 'user_left'
  | 'cursor_update'
  | 'note_updated'
  | 'comment_added'
  | 'error'
  | 'open'
  | 'close'

interface CursorPosition {
  x: number
  y: number
  selection?: string
}

interface NoteEditMessage {
  type: 'note_edit'
  noteId: string
  operation: 'insert' | 'delete'
  position: number
  content: string
}

// Union of all client→server outgoing messages.
type OutgoingMessage =
  | { type: 'join'; projectId: string }
  | { type: 'leave'; projectId: string }
  | { type: 'cursor'; projectId: string; position: CursorPosition }
  | NoteEditMessage
  | { type: 'comment'; noteId: string; content: string; position?: number }

type Handler = (payload: any) => void

const BACKOFF_STEPS = [1000, 2000, 5000]

/**
 * Resolves the configured WS_BASE_URL (which may be a relative path like
 * '/ws') to an absolute ws:// or wss:// URL using the current page location.
 * If WS_BASE_URL is already absolute (e.g. 'ws://localhost:3000/ws'), it is
 * used as-is.
 */
function resolveWsUrl(url: string): string {
  if (/^wss?:\/\//.test(url)) return url
  if (typeof window === 'undefined') return url
  const loc = window.location
  const scheme = loc.protocol === 'https:' ? 'wss:' : 'ws:'
  return `${scheme}//${loc.host}${url.startsWith('/') ? url : `/${url}`}`
}

/**
 * Thin WebSocket wrapper with auto-reconnect and a small event-subscription
 * API. Used by the realtime collaboration features (project rooms, note
 * editing, cursors, comments).
 */
export class RealtimeClient {
  private url: string
  private ws: WebSocket | null = null
  private handlers: Map<RealtimeEvent, Set<Handler>> = new Map()
  private reconnectAttempts = 0
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null
  private shouldReconnect = true
  private joinedProjectIds: Set<string> = new Set()
  private isManualClose = false

  constructor(url: string = WS_BASE_URL) {
    this.url = resolveWsUrl(url)
  }

  connect(): void {
    if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
      return
    }
    this.isManualClose = false
    this.shouldReconnect = true
    try {
      this.ws = new WebSocket(this.url)
    } catch (err) {
      this.scheduleReconnect()
      this.emit('error', { message: err instanceof Error ? err.message : 'Failed to open WebSocket' })
      return
    }

    this.ws.addEventListener('open', this.handleOpen)
    this.ws.addEventListener('message', this.handleMessage)
    this.ws.addEventListener('close', this.handleClose)
    this.ws.addEventListener('error', this.handleError)
  }

  disconnect(): void {
    this.isManualClose = true
    this.shouldReconnect = false
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer)
      this.reconnectTimer = null
    }
    this.joinedProjectIds.clear()
    if (this.ws) {
      this.ws.removeEventListener('open', this.handleOpen)
      this.ws.removeEventListener('message', this.handleMessage)
      this.ws.removeEventListener('close', this.handleClose)
      this.ws.removeEventListener('error', this.handleError)
      try {
        this.ws.close()
      } catch {
        // ignore
      }
      this.ws = null
    }
  }

  private send(message: OutgoingMessage): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      // Drop messages when the socket isn't open; the UI may re-issue them
      // once the connection is re-established.
      return
    }
    try {
      this.ws.send(JSON.stringify(message))
    } catch {
      // swallow — nothing actionable here
    }
  }

  join(projectId: string): void {
    this.joinedProjectIds.add(projectId)
    this.send({ type: 'join', projectId })
  }

  leave(projectId: string): void {
    this.joinedProjectIds.delete(projectId)
    this.send({ type: 'leave', projectId })
  }

  sendCursor(projectId: string, position: CursorPosition): void {
    this.send({ type: 'cursor', projectId, position })
  }

  sendNoteEdit(
    noteId: string,
    operation: 'insert' | 'delete',
    position: number,
    content: string,
  ): void {
    this.send({ type: 'note_edit', noteId, operation, position, content })
  }

  sendComment(noteId: string, content: string, position?: number): void {
    this.send({ type: 'comment', noteId, content, position })
  }

  on(event: RealtimeEvent, handler: Handler): void {
    let set = this.handlers.get(event)
    if (!set) {
      set = new Set()
      this.handlers.set(event, set)
    }
    set.add(handler)
  }

  off(event: RealtimeEvent, handler: Handler): void {
    const set = this.handlers.get(event)
    if (set) set.delete(handler)
  }

  private emit(event: RealtimeEvent, payload?: any): void {
    const set = this.handlers.get(event)
    if (!set) return
    set.forEach((h) => {
      try {
        h(payload)
      } catch {
        // handler errors shouldn't tear down the socket
      }
    })
  }

  private handleOpen = () => {
    this.reconnectAttempts = 0
    // Re-join any project rooms we were in before the (re)connect.
    this.joinedProjectIds.forEach((projectId) => {
      this.send({ type: 'join', projectId })
    })
    this.emit('open')
  }

  private handleMessage = (event: MessageEvent) => {
    let data: any
    try {
      data = JSON.parse(typeof event.data === 'string' ? event.data : '{}')
    } catch {
      return
    }
    if (!data || typeof data.type !== 'string') return
    const evtType = data.type as RealtimeEvent
    this.emit(evtType, data)
  }

  private handleClose = () => {
    this.emit('close')
    if (this.shouldReconnect && !this.isManualClose) {
      this.scheduleReconnect()
    }
  }

  private handleError = (ev: Event) => {
    // The browser doesn't expose the underlying error message via the
    // WebSocket error event; emit a generic message and let the close
    // handler trigger reconnect.
    this.emit('error', { message: 'WebSocket error', event: ev })
  }

  private scheduleReconnect() {
    if (this.reconnectTimer) return
    const delay = BACKOFF_STEPS[Math.min(this.reconnectAttempts, BACKOFF_STEPS.length - 1)]
    this.reconnectAttempts += 1
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null
      this.connect()
    }, delay)
  }
}

export const realtime = new RealtimeClient()
