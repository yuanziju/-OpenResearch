import { useEffect, useState } from 'react'
import { papersApi } from '@/client/api/client'
import { notesApi } from '@/client/api/client'
import { projectsApi } from '@/client/api/client'

interface StatCardProps {
  label: string
  value: number | string
}

const headingStyle: React.CSSProperties = {
  fontSize: '28px',
  color: '#e0e0e0',
  margin: '0 0 8px 0',
}

const subtitleStyle: React.CSSProperties = {
  fontSize: '16px',
  color: '#a0a0b0',
  margin: '0 0 32px 0',
}

const cardsRowStyle: React.CSSProperties = {
  display: 'flex',
  gap: '16px',
  flexWrap: 'wrap',
}

const cardStyle: React.CSSProperties = {
  background: '#16213e',
  padding: '20px',
  borderRadius: '8px',
  minWidth: '160px',
}

const cardLabelStyle: React.CSSProperties = {
  fontSize: '14px',
  color: '#a0a0b0',
  marginBottom: '8px',
}

const cardValueStyle: React.CSSProperties = {
  fontSize: '32px',
  fontWeight: 'bold',
  color: '#ffffff',
}

const errorStyle: React.CSSProperties = {
  color: '#e94560',
  background: '#2a1a2a',
  padding: '10px 14px',
  borderRadius: '6px',
  marginBottom: '16px',
  fontSize: '14px',
}

function StatCard({ label, value }: StatCardProps) {
  return (
    <div style={cardStyle}>
      <div style={cardLabelStyle}>{label}</div>
      <div style={cardValueStyle}>{value}</div>
    </div>
  )
}

interface Counts {
  papers: number
  notes: number
  projects: number
}

function Dashboard() {
  const [counts, setCounts] = useState<Counts>({ papers: 0, notes: 0, projects: 0 })
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    let cancelled = false
    async function load() {
      setLoading(true)
      setError(null)
      try {
        // Fire the three independent count requests in parallel. Each request
        // surfaces a total / list length we can use for the stat card.
        const [papersRes, notesRes, projectsRes] = await Promise.all([
          papersApi.list({ page: 1, limit: 1 }),
          notesApi.list(),
          projectsApi.list(),
        ])
        if (cancelled) return
        setCounts({
          papers: papersRes.total,
          notes: notesRes.notes.length,
          projects: projectsRes.projects.length,
        })
      } catch (err) {
        if (cancelled) return
        setError(err instanceof Error ? err.message : 'Failed to load dashboard data')
      } finally {
        if (!cancelled) setLoading(false)
      }
    }
    load()
    return () => {
      cancelled = true
    }
  }, [])

  return (
    <div>
      <h1 style={headingStyle}>Welcome to Open Research</h1>
      <p style={subtitleStyle}>
        An open-source research tool that redefines the research workflow.
      </p>
      {error && <div style={errorStyle}>{error}</div>}
      <div style={cardsRowStyle}>
        <StatCard label="Total Papers" value={loading ? '…' : counts.papers} />
        <StatCard label="Recent Notes" value={loading ? '…' : counts.notes} />
        <StatCard label="Active Projects" value={loading ? '…' : counts.projects} />
      </div>
    </div>
  )
}

export default Dashboard
